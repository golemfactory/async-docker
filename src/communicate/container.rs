use hyper::client::connect::Connect;
use std::borrow::Cow;
use futures::Future;
use representation::rep::ContainerDetails;
use Error;
use Result;
use futures::Stream;
use build::LogsOptions;

use docker::DockerTrait;
use util::build_simple_query;

use transport::parse::parse_to_lines;
use transport::parse::parse_to_trait;
use transport::parse::parse_to_stream;
use transport::parse::status_code;

use representation::rep::Change;
use representation::rep::Stats;
use representation::rep::Top;
use transport::parse::parse_to_file;
use http::StatusCode;
use hyper::Body;
use std::time::Duration;
use representation::rep::Exit;
use build::RmContainerOptions;
use build::ExecContainerOptions;
use serde_json::Value;
use errors::ErrorKind as EK;
use transport::tty;
use futures::future;
use hyper::Chunk;
use transport::interact::Interact;
use std::sync::Arc;

/// Interface for accessing and manipulating a docker container
pub struct Container<T>
    where
        T: 'static + Connect,
{
    interact: Arc<Interact<T>>,
    id: Cow<'static, str>,
}

impl<T> Clone for Container<T>
    where
        T: 'static + Connect,
{
    fn clone(&self) -> Container<T> {
        let interact = self.interact.clone();
        Container::new(self.interact.clone(), self.id.clone())
    }
}

impl<T> Container<T>
    where
        T: 'static + Connect,
{
    /// Exports an interface exposing operations against a container instance
    pub(crate) fn new(interact: Arc<Interact<T>>, id: Cow<'static, str>) -> Container<T>
    {
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
    pub fn inspect(&self) -> impl Future<Item=ContainerDetails, Error=Error> + Send {
        let path = Some(format!("/containers/{}/json", self.id));
        let query : Option<&str> = None;

        parse_to_trait::<ContainerDetails>(self.interact.get(path, query))
    }

    /// Returns a `top` view of information about the container process
    pub fn top(&self, psargs: Option<&str>) -> impl Future<Item=Top, Error=Error> + Send {
        let path = Some(format!("/containers/{}/top", self.id));
        let query = build_simple_query("ps_args", psargs);

        parse_to_trait::<Top>(self.interact.get(path, query))
    }

    /// Returns a stream of logs emitted but the container instance
    pub fn logs(&self, opts: &LogsOptions) -> impl Stream<Item=String, Error=Error> + Send {
        let path = Some(format!("/containers/{}/logs", self.id));
        let query = opts.serialize();

        parse_to_lines(self.interact.get(path, query))
    }

    /// Returns a set of changes made to the container instance
    pub fn changes(&self) -> impl Future<Item=Vec<Change>, Error=Error> + Send {
        let path = Some(format!("/containers/{}/changes", self.id));
        let query : Option<&str>  = None;

        parse_to_trait::<Vec<Change>>(self.interact.get(path, query))
    }

    /// Exports the current docker container into a tarball
    pub fn export(&self) -> impl Future<Item=(), Error=Error> + Send {
        let path = Some(format!("/containers/{}/export", self.id));
        let query : Option<&str>  = None;

        parse_to_file(self.interact.get(path, query), "antonn")
    }

    /// Returns a stream of stats specific to this container instance
    pub fn stats(&self) -> impl Stream<Item=Result<Stats>, Error=Error> + Send {
        let path = Some(format!("/containers/{}/stats", self.id));
        let query : Option<&str> = None;

        parse_to_stream::<Stats>(self.interact.get(path, query))
    }

    /// Start the container instance
    pub fn start(&self) ->  impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}/start", self.id));
        let query : Option<&str> = None;
        let body : Option<Body>  = None;

        status_code(self.interact.post(path, query))
    }

    /// Stop the container instance
    pub fn stop(&self, wait: Option<Duration>) -> impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}/start", self.id));
        let query =
            build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let body : Option<Body>  = None;


        status_code(self.interact.post(path, query))
    }

    /// Restart the container instance
    pub fn restart(&self, wait: Option<Duration>) -> impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}/restart", self.id));
        let query =
            build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let body : Option<Body>  = None;

        status_code(self.interact.post(path, query))
    }

    /// Kill the container instance
    pub fn kill(&self, signal: Option<&str>) -> impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}/kill", self.id));
        let query =
            build_simple_query("signal", signal.map(|sig| sig));
        let body : Option<Body>  = None;

        status_code(self.interact.post(path, query))
    }

    /// Rename the container instance
    pub fn rename(&self, name: &str) -> impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}/rename", self.id));
        let query =
            build_simple_query("name", Some(name));
        let body : Option<Body>  = None;

        status_code(self.interact.post(path, query))
    }

    /// Pause the container instance
    pub fn pause(&self) -> impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}/pause", self.id));
        let query : Option<String> = None;
        let body : Option<Body>  = None;

        status_code(self.interact.post(path, query))
    }

    /// Unpause the container instance
    pub fn unpause(&self) -> impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}/unpause", self.id));
        let query: Option<String> = None;
        let body: Option<Body> = None;

        status_code(self.interact.post(path, query))
    }

    /// Wait until the container stops
    pub fn wait(&self) -> impl Future<Item=Exit, Error=Error> + Send {
        let path = Some(format!("/containers/{}/wait", self.id));
        let query: Option<String> = None;
        let body: Option<Body> = None;

        parse_to_trait::<Exit>(self.interact.post(path, query))
    }

    /// Delete the container instance
    ///
    /// Use remove instead to use the force/v options.
    pub fn delete(&self) -> impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}", self.id));
        let query: Option<String> = None;
        let body: Option<Body> = None;

        status_code(self.interact.delete(path, query))
    }

    /// Delete the container instance (todo: force/v)
    pub fn remove(&self, opts: &RmContainerOptions) -> impl Future<Item=StatusCode, Error=Error> + Send {
        let path = Some(format!("/containers/{}", self.id));
        let query = opts.serialize();
        let body: Option<Body> = None;

        status_code(self.interact.delete(path, query))
    }

    pub fn create_exec(&self, opts: &ExecContainerOptions)
        -> impl Future<Item=String, Error=Error> + Send
    {
        let path = Some(format!("/containers/{}/exec", self.id));
        let query: Option<String> = None;
        let body = opts.serialize();

        debug!("Body: {:?}", body);

        parse_to_trait::<Value>(self.interact._post_json(path, query, body))
            .and_then(|val| {
                debug!("{:?}", val);
                match val {
                    Value::Object(obj) => future::result(obj
                        .get("Id")
                        .ok_or(Error::from(EK::JsonFieldMissing("Id")))
                        .and_then(|val| val.as_str()
                            .ok_or(Error::from(EK::JsonTypeError("Id", "String"))))
                        .map(|a| {
                            debug!("String: {}", a.to_string());
                            a.to_string()
                        })),
                    _ => future::err(Error::from(EK::JsonTypeError("<anonymous>", "Object")))
                }
            })
    }

    pub fn start_exec(&self, id: String)
        -> impl Stream<Item=(u32, Chunk), Error=Error>
    {
        debug!("Start exec");
        debug!("{:?}", id);
        let path = Some(format!("/exec/{}/start", id));
        let query: Option<String> = None;
        let body : Option<String> = Some("{}".to_string());

        let body_future = self.interact.post_json(path, query, body)
            .and_then(|response| {
                debug!("exec 1");
                response.map_err(Error::from)
            })
            .and_then(|result| {
                debug!("exec 2");
                Ok(result.into_body().map_err(Error::from))
            })
            .flatten_stream();

        tty::decode(body_future)
/*
        let path = Some(format!("/exec/{}/start", id));
        let query : Option<String> = None;
        let body : Option<String> = Some("{}".to_string());
        self.interact.post_json(path, query, body)
            .and_then(|future| {
                debug!(">>> future");
                future.map_err(Error::from)
            })
            .and_then(|response| {
                debug!(">>> Response");
                debug!("{:?}", response.headers());
                future::ok(tty::decode(response.into_body().map_err(Error::from)))
            })
            .flatten_stream()
*/  }

    pub fn exec(&self, opts: &ExecContainerOptions)
        -> impl Stream<Item=(u32, Chunk), Error=Error>
    {
        let copy_self = self.clone();

        self.create_exec(opts)
            .and_then(move |id| Ok(copy_self.start_exec(id)))
            .flatten_stream()
    }

    /*
    pub fn archive_put(&self, opts: &ContainerArchiveOptions) -> Result<()> {
        let mut path = vec![(&format!("/containers/{}/archive", self.id)).to_owned()];

        if let Some(query) = opts.serialize() {
            path.push(query);
        }

        let mut bytes = vec![];

        tarball::dir(&mut bytes, &opts.local_path)?;

        let body = Body::BufBody(&bytes[..], bytes.len());

        self.docker
            .stream_put(&path.join("?"), Some(body))
            .map(|_| ())
    }
    */

    // todo attach, attach/ws, copy
}