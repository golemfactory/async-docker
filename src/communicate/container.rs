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
pub struct Container<'b, T>
    where
        T: 'static + Connect,
{
    interact: Arc<Interact<T>>,
    id: Cow<'b, str>,
}

impl<'b, T> Container<'b, T>
    where
        T: 'static + Connect,
{
    /// Exports an interface exposing operations against a container instance
    pub(crate) fn new(interact: Arc<Interact<T>>, id: Cow<'b, str>) -> Container<'b, T>
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
    pub fn inspect(&self) -> Box<Future<Item=ContainerDetails, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/json", self.id));
        let query : Option<&str> = None;

        Box::new(parse_to_trait::<ContainerDetails>(self.interact.get(path, query)))
    }

    /// Returns a `top` view of information about the container process
    pub fn top(&self, psargs: Option<&str>) -> Box<Future<Item=Top, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/top", self.id));
        let query = build_simple_query("ps_args", psargs);

        Box::new(parse_to_trait::<Top>(self.interact.get(path, query)))
    }

    /// Returns a stream of logs emitted but the container instance
    pub fn logs(&self, opts: &LogsOptions) -> Box<Stream<Item=String, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/logs", self.id));
        let query = opts.serialize();

        Box::new(parse_to_lines(self.interact.get(path, query)))
    }

    /// Returns a set of changes made to the container instance
    pub fn changes(&self) -> Box<Future<Item=Vec<Change>, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/changes", self.id));
        let query : Option<&str>  = None;

        Box::new(parse_to_trait::<Vec<Change>>(self.interact.get(path, query)))
    }

    /// Exports the current docker container into a tarball
    pub fn export(&self) -> Box<Future<Item=(), Error=Error> + Send> {
        let path = Some(format!("/containers/{}/export", self.id));
        let query : Option<&str>  = None;

        Box::new(parse_to_file(self.interact.get(path, query), "antonn"))
    }

    /// Returns a stream of stats specific to this container instance
    pub fn stats(&self) -> Box<Stream<Item=Result<Stats>, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/stats", self.id));
        let query : Option<&str> = None;

        Box::new(parse_to_stream::<Stats>(self.interact.get(path, query)))
    }

    /// Start the container instance
    pub fn start(&self) ->  Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/start", self.id));
        let query : Option<&str> = None;
        let body : Option<Body>  = None;

        Box::new(status_code(self.interact.post(path, query, body)))
    }

    /// Stop the container instance
    pub fn stop(&self, wait: Option<Duration>) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/start", self.id));
        let query =
            build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let body : Option<Body>  = None;


        Box::new(status_code(self.interact.post(path, query, body)))
    }

    /// Restart the container instance
    pub fn restart(&self, wait: Option<Duration>) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/restart", self.id));
        let query =
            build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let body : Option<Body>  = None;

        Box::new(status_code(self.interact.post(path, query, body)))
    }

    /// Kill the container instance
    pub fn kill(&self, signal: Option<&str>) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/kill", self.id));
        let query =
            build_simple_query("signal", signal.map(|sig| sig));
        let body : Option<Body>  = None;

        Box::new(status_code(self.interact.post(path, query, body)))
    }

    /// Rename the container instance
    pub fn rename(&self, name: &str) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/rename", self.id));
        let query =
            build_simple_query("name", Some(name));
        let body : Option<Body>  = None;

        Box::new(status_code(self.interact.post(path, query, body)))
    }

    /// Pause the container instance
    pub fn pause(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/pause", self.id));
        let query : Option<String> = None;
        let body : Option<Body>  = None;

        Box::new(status_code(self.interact.post(path, query, body)))
    }

    /// Unpause the container instance
    pub fn unpause(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/unpause", self.id));
        let query: Option<String> = None;
        let body: Option<Body> = None;

        Box::new(status_code(self.interact.post(path, query, body)))
    }

    /// Wait until the container stops
    pub fn wait(&self) -> Box<Future<Item=Exit, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/unpause", self.id));
        let query: Option<String> = None;
        let body: Option<Body> = None;

        Box::new(parse_to_trait::<Exit>(self.interact.post(path, query, body)))
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

    /// Exec the specified command in the container
    pub fn exec(&self, opts: &ExecContainerOptions) -> impl Stream<Item=(u32, Chunk), Error=Error> + Send {
        let path = Some(format!("/containers/{}/exec", self.id));
        let query: Option<String> = None;
        let body = opts.serialize();
        let interact = self.interact.clone();

        println!("{:?}", body);


        parse_to_trait::<Value>(self.interact.post(path, query, body))
            .and_then(|val| {

                println!(">>> {:?}", val);
                match val {
                    Value::Object(obj) => future::result(obj
                        .get("Id")
                        .ok_or(Error::from(EK::JsonFieldMissing("Id")))
                        .and_then(|val| val.as_str()
                            .ok_or(Error::from(EK::JsonTypeError("Id", "String"))))
                        .map(|a| a.to_string())),
                    _ => future::err(Error::from(EK::JsonTypeError("<anonymous>", "Object")))
                }
            })
            .and_then(move |id| {
                let path = Some(format!("/containers/{}/start", id));
                let query : Option<String> = None;
                let body : Option<Body> = None;

                let stream =
                    interact.post(path, query, body)
                    .and_then(|future| future.map_err(Error::from))
                    .and_then(|response| {
                        future::ok(tty::decode(response.into_body().map_err(Error::from)))
                    });

                stream
            })
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