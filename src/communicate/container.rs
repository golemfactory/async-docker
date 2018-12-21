use build::LogsOptions;
use futures::{Future, Stream};
use std::borrow::Cow;
use Error;
use Result;

use transport::parse::{empty_result2, parse_to_lines, parse_to_stream, parse_to_trait};
use util::build_simple_query;

use build::{ContainerArchivePutOptions, RmContainerOptions};
use communicate::util::AsSlice;
use futures::future;
use http::StatusCode;
use hyper::{Body, Chunk};
use models::ContainerChangeResponseItem;
use models::{ContainerConfig, ContainerTopResponse, ExecConfig, IdResponse};
use representation::rep::Stats;
use std::{sync::Arc, time::Duration};
use tarball::tarball;
use transport::{
    interact::{InteractApi, InteractApiExt},
    tty,
};
use serde_json::Value;

/// Interface for accessing and manipulating a docker container
pub struct Container {
    interact: Arc<InteractApi>,
    id: Cow<'static, str>,
}

impl Clone for Container {
    fn clone(&self) -> Container {
        Container::new(self.interact.clone(), self.id.clone())
    }
}

impl Container {
    /// Exports an interface exposing operations against a container instance
    pub(crate) fn new(interact: Arc<InteractApi>, id: Cow<'static, str>) -> Container {
        Container {
            interact,
            id: id.into(),
        }
    }

    /// a getter for the container id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Inspects the current docker container instance's details
    pub fn inspect(&self) -> impl Future<Item = ContainerConfig, Error = Error> + Send {
        let path = format!("/containers/{}/json", self.id);
        let args = path.as_str();

        parse_to_trait::<ContainerConfig>(self.interact.get(args))
    }

    /// Returns a `top` view of information about the container process
    pub fn top(
        &self,
        psargs: Option<&str>,
    ) -> impl Future<Item = ContainerTopResponse, Error = Error> + Send {
        let path = format!("/containers/{}/top", self.id);
        let query = build_simple_query("ps_args", psargs);
        let args = (path.as_ref(), query.as_slice());

        parse_to_trait::<ContainerTopResponse>(self.interact.get(args))
    }

    /// Returns a stream of logs emitted but the container instance
    pub fn logs(&self, opts: &LogsOptions) -> impl Stream<Item = String, Error = Error> + Send {
        let path = format!("/containers/{}/logs", self.id);
        let query = opts.serialize();
        let args = (path.as_str(), query.as_slice());

        parse_to_lines(self.interact.get(args))
    }

    /// Returns a set of changes made to the container instance
    pub fn changes(
        &self,
    ) -> impl Future<Item = Vec<ContainerChangeResponseItem>, Error = Error> + Send {
        let args = format!("/containers/{}/changes", self.id);

        parse_to_trait::<Vec<ContainerChangeResponseItem>>(self.interact.get(args.as_str()))
    }

    /// Exports the current docker container into a tarball
    pub fn export(&self) -> impl Stream<Item = Chunk, Error = Error> + Send {
        let path = format!("/containers/{}/export", self.id);

        self.interact
            .get(path.as_str())
            .and_then(|a| a.map_err(Error::from))
            .and_then(|a| Ok(a.into_body().map_err(Error::from)))
            .flatten_stream()
    }

    /// Returns a stream of stats specific to this container instance
    pub fn stats(&self) -> impl Stream<Item = Result<Stats>, Error = Error> + Send {
        let args = format!("/containers/{}/stats", self.id);

        parse_to_stream::<Stats>(self.interact.get(args.as_str()))
    }

    /// Start the container instance
    pub fn start(&self) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let args = format!("/containers/{}/start", self.id);

        empty_result2(self.interact.post(args.as_str()))
    }

    /// Stop the container instance
    pub fn stop(&self, wait: Option<Duration>) -> impl Future<Item = (), Error = Error> + Send {
        let path = format!("/containers/{}/stop", self.id);
        let query = build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let args = (path.as_str(), query.as_slice());

        parse_to_trait(self.interact.post(args))
    }

    /// Restart the container instance
    pub fn restart(&self, wait: Option<Duration>) -> impl Future<Item = (), Error = Error> + Send {
        let path = format!("/containers/{}/restart", self.id);
        let query = build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let args = (path.as_str(), query.as_slice());

        parse_to_trait(self.interact.post(args))
    }

    /// Kill the container instance
    pub fn kill(&self, signal: Option<&str>) -> impl Future<Item = (), Error = Error> + Send {
        let path = format!("/containers/{}/kill", self.id);
        let query = build_simple_query("signal", signal.map(|sig| sig));
        let args = (path.as_str(), query.as_slice());

        parse_to_trait(self.interact.post(args))
    }

    /// Rename the container instance
    pub fn rename(&self, name: &str) -> impl Future<Item = (), Error = Error> + Send {
        let path = format!("/containers/{}/rename", self.id);
        let query = build_simple_query("name", Some(name));
        let args = (path.as_str(), query.as_slice());

        parse_to_trait(self.interact.post(args))
    }

    /// Pause the container instance
    pub fn pause(&self) -> impl Future<Item = (), Error = Error> + Send {
        let args = format!("/containers/{}/pause", self.id);

        parse_to_trait(self.interact.post(args.as_str()))
    }

    /// Unpause the container instance
    pub fn unpause(&self) -> impl Future<Item = (), Error = Error> + Send {
        let args = format!("/containers/{}/unpause", self.id);

        parse_to_trait(self.interact.post(args.as_str()))
    }

    /// Wait until the container stops
    pub fn wait(&self) -> impl Future<Item = u8, Error = Error> + Send {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct WaitResult {
            _error: Value,
            status_code: u8,
        }

        let args = format!("/containers/{}/wait", self.id);

        parse_to_trait::<WaitResult>(self.interact.post(args.as_str()))
            .map(|res| res.status_code)
    }

    /// Delete the container instance
    ///
    /// Use remove instead to use the force/v options.
    pub fn delete(&self) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let args = format!("/containers/{}", self.id);

        empty_result2(self.interact.delete(args.as_str()))
    }

    /// Delete the container instance (todo: force/v)
    pub fn remove(
        &self,
        opts: &RmContainerOptions,
    ) -> impl Future<Item = (), Error = Error> + Send {
        let path = format!("/containers/{}", self.id);
        let query = opts.serialize();
        let args = (path.as_str(), query.as_slice());

        parse_to_trait(self.interact.delete(args))
    }

    pub fn create_exec(
        &self,
        opts: &ExecConfig,
    ) -> impl Future<Error = Error, Item = IdResponse> + Send {
        let path = format!("/containers/{}/exec", self.id);
        let body = serde_json::ser::to_string(opts).map(|s| Body::from(s)).ok();
        let args = (path.as_str(), body);

        parse_to_trait(self.interact.post_json(args))
    }

    pub fn start_exec(&self, id: String) -> impl Stream<Item = (u32, Chunk), Error = Error> {
        let path = format!("/exec/{}/start", id);
        let body = Some(Body::from("{}".to_string()));
        let args = (path.as_str(), body);

        let body_future = self
            .interact
            .post_json(args)
            .and_then(|response| response.map_err(Error::from))
            .and_then(|result| Ok(result.into_body().map_err(Error::from)))
            .flatten_stream();

        tty::decode(body_future)
    }

    pub fn exec(&self, opts: &ExecConfig) -> impl Stream<Item = (u32, Chunk), Error = Error> {
        let copy_self = self.clone();
        self.create_exec(opts)
            .and_then(move |res| Ok(copy_self.start_exec(res.id().to_owned())))
            .flatten_stream()
    }

    pub fn archive_get(&self, pth: &str) -> impl Stream<Item = Chunk, Error = Error> {
        let path = format!("/containers/{}/archive", self.id);
        let query = build_simple_query("path", Some(pth));
        let args = (path.as_str(), query.as_slice());

        self.interact
            .get(args)
            .and_then(|a| a.map_err(Error::from))
            .and_then(|a| Ok(a.into_body().map_err(Error::from)))
            .flatten_stream()
    }

    pub fn archive_put(
        &self,
        opts: &ContainerArchivePutOptions,
    ) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let mut bytes = vec![];
        let path = format!("/containers/{}/archive", self.id);
        let query = opts.serialize();
        let interact = self.interact.clone();

        future::result(tarball::dir(&mut bytes, &opts.local_path)).and_then(move |_| {
            let body = Some(Body::from(bytes));
            let args = (path.as_str(), query.as_slice(), body);
            empty_result2(interact.put(args))
        })
    }

    // todo attach, attach/ws, copy
}
