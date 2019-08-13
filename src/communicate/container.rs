use build::LogsOptions;
use futures::Future;
use futures::Stream;
use representation::rep::ContainerDetails;
use std::borrow::Cow;
use Error;
use Result;

use util::build_simple_query;

use transport::parse::parse_to_lines;
use transport::parse::parse_to_stream;
use transport::parse::parse_to_trait;
use transport::parse::status_code;

use build::ContainerArchivePutOptions;
use build::ExecContainerOptions;
use build::RmContainerOptions;
use communicate::util::AsSlice;
use errors::ErrorKind as EK;
use futures::future;
use http::StatusCode;
use hyper::Body;
use hyper::Chunk;
use representation::rep::Change;
use representation::rep::Exit;
use representation::rep::Stats;
use representation::rep::Top;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tarball::tarball;
use transport::interact::InteractApi;
use transport::interact::InteractApiExt;
use transport::tty;

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
    pub fn inspect(&self) -> impl Future<Item = ContainerDetails, Error = Error> + Send {
        let path = format!("/containers/{}/json", self.id);
        let args = path.as_str();

        parse_to_trait::<ContainerDetails>(self.interact.get(args))
    }

    /// Returns a `top` view of information about the container process
    pub fn top(&self, psargs: Option<&str>) -> impl Future<Item = Top, Error = Error> + Send {
        let path = format!("/containers/{}/top", self.id);
        let query = build_simple_query("ps_args", psargs);
        let args = (path.as_ref(), query.as_slice());

        parse_to_trait::<Top>(self.interact.get(args))
    }

    /// Returns a stream of logs emitted but the container instance
    pub fn logs(&self, opts: &LogsOptions) -> impl Stream<Item = String, Error = Error> + Send {
        let path = format!("/containers/{}/logs", self.id);
        let query = opts.serialize();
        let args = (path.as_str(), query.as_slice());

        parse_to_lines(self.interact.get(args))
    }

    /// Returns a set of changes made to the container instance
    pub fn changes(&self) -> impl Future<Item = Vec<Change>, Error = Error> + Send {
        let args = format!("/containers/{}/changes", self.id);

        parse_to_trait::<Vec<Change>>(self.interact.get(args.as_str()))
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

        status_code(self.interact.post(args.as_str()))
    }

    /// Stop the container instance
    pub fn stop(
        &self,
        wait: Option<Duration>,
    ) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let path = format!("/containers/{}/stop", self.id);
        let query = build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.post(args))
    }

    /// Restart the container instance
    pub fn restart(
        &self,
        wait: Option<Duration>,
    ) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let path = format!("/containers/{}/restart", self.id);
        let query = build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.post(args))
    }

    /// Kill the container instance
    pub fn kill(
        &self,
        signal: Option<&str>,
    ) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let path = format!("/containers/{}/kill", self.id);
        let query = build_simple_query("signal", signal.map(|sig| sig));
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.post(args))
    }

    /// Rename the container instance
    pub fn rename(&self, name: &str) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let path = format!("/containers/{}/rename", self.id);
        let query = build_simple_query("name", Some(name));
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.post(args))
    }

    /// Pause the container instance
    pub fn pause(&self) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let args = format!("/containers/{}/pause", self.id);

        status_code(self.interact.post(args.as_str()))
    }

    /// Unpause the container instance
    pub fn unpause(&self) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let args = format!("/containers/{}/unpause", self.id);

        status_code(self.interact.post(args.as_str()))
    }

    /// Wait until the container stops
    pub fn wait(&self) -> impl Future<Item = Exit, Error = Error> + Send {
        let args = format!("/containers/{}/wait", self.id);

        parse_to_trait::<Exit>(self.interact.post(args.as_str()))
    }

    /// Delete the container instance
    ///
    /// Use remove instead to use the force/v options.
    pub fn delete(&self) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let args = format!("/containers/{}", self.id);

        status_code(self.interact.delete(args.as_str()))
    }

    /// Delete the container instance (todo: force/v)
    pub fn remove(
        &self,
        opts: &RmContainerOptions,
    ) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let path = format!("/containers/{}", self.id);
        let query = opts.serialize();
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.delete(args))
    }

    pub fn create_exec(
        &self,
        opts: &ExecContainerOptions,
    ) -> impl Future<Item = String, Error = Error> + Send {
        let path = format!("/containers/{}/exec", self.id);
        let body = opts.serialize().map(Body::from);
        let args = (path.as_str(), body);

        parse_to_trait::<Value>(self.interact.post_json(args)).and_then(|val| {
            debug!("{:?}", val);
            match val {
                Value::Object(obj) => future::result(
                    obj.get("Id")
                        .ok_or(Error::from(EK::JsonFieldMissing("Id")))
                        .and_then(|val| {
                            val.as_str()
                                .ok_or(Error::from(EK::JsonTypeError("Id", "String")))
                        }).map(|a| {
                            debug!("String: {}", a.to_string());
                            a.to_string()
                        }),
                ),
                _ => future::err(Error::from(EK::JsonTypeError("<anonymous>", "Object"))),
            }
        })
    }

    /* TODO */
    pub fn check_exec_status(&self, id: String) -> impl Future<Item = String, Error = Error> {
        let path = format!("/exec/{}/json", id);
        self.interact.post_json((path.as_str(), Option::<&str>::None))
            .and_then(|val| {
                future::ok("0".to_string()) /* TODO */
            })
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

    pub fn exec(
        &self,
        opts: &ExecContainerOptions,
    ) -> impl Stream<Item = (u32, Chunk), Error = Error> {
        let copy_self = self.clone();
        //let copy_self_2 = self.clone();
        self.create_exec(opts)
            .and_then(move |id| {
                future::ok(copy_self.start_exec(id.clone())).join(future::ok(id))
            })
            /*.and_then(|(stream, id)| {
                (copy_self_2.check_exec_status(id)).join(future::ok(stream))
            })*/
            .and_then(|(stream, _status)| {
                future::ok(stream)
            })
            .flatten_stream()
/*          .and_then(move |id| Ok(copy_self.start_exec(id)))
            .flatten_stream()
            */
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
            status_code(interact.put(args))
        })
    }

    // todo attach, attach/ws, copy
}
