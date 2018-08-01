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

/// Interface for accessing and manipulating a docker container
pub struct Container<'a, 'b, D, T>
    where
        D: 'a + DockerTrait<Connector=T>,
        T: 'static + Connect,
{
    docker: &'a D,
    id: Cow<'b, str>,
}

impl<'a, 'b, D, T> Container<'a, 'b, D, T>
    where
        D: DockerTrait<Connector=T>,
        T: 'static + Connect,
{
    /// Exports an interface exposing operations against a container instance
    pub fn new<S>(docker: &'a D, id: S) -> Container<'a, 'b, D, T>
        where
            S: Into<Cow<'b, str>>,
    {
        Container {
            docker,
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

        Box::new(parse_to_trait::<ContainerDetails>(self.docker.get(path, query)))
    }

    /// Returns a `top` view of information about the container process
    pub fn top(&self, psargs: Option<&str>) -> Box<Future<Item=Top, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/top", self.id));
        let query = build_simple_query("ps_args", psargs);

        Box::new(parse_to_trait::<Top>(self.docker.get(path, query)))
    }

    /// Returns a stream of logs emitted but the container instance
    pub fn logs(&self, opts: &LogsOptions) -> Box<Stream<Item=String, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/logs", self.id));
        let query = opts.serialize();

        Box::new(parse_to_lines(self.docker.get(path, query)))
    }

    /// Returns a set of changes made to the container instance
    pub fn changes(&self) -> Box<Future<Item=Vec<Change>, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/changes", self.id));
        let query : Option<&str>  = None;

        Box::new(parse_to_trait::<Vec<Change>>(self.docker.get(path, query)))
    }

    /// Exports the current docker container into a tarball
    pub fn export(&self) -> Box<Future<Item=(), Error=Error> + Send> {
        let path = Some(format!("/containers/{}/export", self.id));
        let query : Option<&str>  = None;

        Box::new(parse_to_file(self.docker.get(path, query), "antonn"))
    }

    /// Returns a stream of stats specific to this container instance
    pub fn stats(&self) -> Box<Stream<Item=Result<Stats>, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/stats", self.id));
        let query : Option<&str> = None;

        Box::new(parse_to_stream::<Stats>(self.docker.get(path, query)))
    }

    /// Start the container instance
    pub fn start(&self) ->  Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/start", self.id));
        let query : Option<&str> = None;
        let body : Option<Body>  = None;

        Box::new(status_code(self.docker.post(path, query, body)))
    }

    /// Stop the container instance
    pub fn stop(&self, wait: Option<Duration>) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/start", self.id));
        let query =
            build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let body : Option<Body>  = None;


        Box::new(status_code(self.docker.post(path, query, body)))
    }

    /// Restart the container instance
    pub fn restart(&self, wait: Option<Duration>) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/restart", self.id));
        let query =
            build_simple_query("t", wait.map(|w| w.as_secs().to_string()));
        let body : Option<Body>  = None;

        Box::new(status_code(self.docker.post(path, query, body)))
    }

    /// Kill the container instance
    pub fn kill(&self, signal: Option<&str>) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/kill", self.id));
        let query =
            build_simple_query("signal", signal.map(|sig| sig));
        let body : Option<Body>  = None;

        Box::new(status_code(self.docker.post(path, query, body)))
    }

    /// Rename the container instance
    pub fn rename(&self, name: &str) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/rename", self.id));
        let query =
            build_simple_query("name", Some(name));
        let body : Option<Body>  = None;

        Box::new(status_code(self.docker.post(path, query, body)))
    }

    /// Pause the container instance
    pub fn pause(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/pause", self.id));
        let query : Option<String> = None;
        let body : Option<Body>  = None;

        Box::new(status_code(self.docker.post(path, query, body)))
    }

    /// Unpause the container instance
    pub fn unpause(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/unpause", self.id));
        let query: Option<String> = None;
        let body: Option<Body> = None;

        Box::new(status_code(self.docker.post(path, query, body)))
    }

    /// Wait until the container stops
    pub fn wait(&self) -> Box<Future<Item=Exit, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/unpause", self.id));
        let query: Option<String> = None;
        let body: Option<Body> = None;

        Box::new(parse_to_trait::<Exit>(self.docker.post(path, query, body)))
    }

    /// Delete the container instance
    ///
    /// Use remove instead to use the force/v options.
    pub fn delete(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}", self.id));
        let query: Option<String> = None;
        let body: Option<Body> = None;

        Box::new(status_code(self.docker.delete(path, query)))
    }

    /// Delete the container instance (todo: force/v)
    pub fn remove(&self, opts: &RmContainerOptions) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some(format!("/containers/{}", self.id));
        let query = opts.serialize();
        let body: Option<Body> = None;

        Box::new(status_code(self.docker.delete(path, query)))
    }
/*
    /// Exec the specified command in the container
    pub fn exec(&self, opts: &ExecContainerOptions) -> Box<Future<Item=Tty, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/exec", self.id));
        let query: Option<String> = opts.serialize();
        let body = opts.serialize().ok().map(as_bytes).map(Body::from);


        match self.docker.post(s, Some(&mut bytes)) {
            Err(e) => Err(e),
            Ok(res) => {
                let data = "{}";
                let mut bytes = data.as_bytes();
                let json = ::serde_json::from_str::<Value>(res.as_str())?;

                if let Value::Object(ref _obj) = json {
                    let id = json
                        .get("Id")
                        .ok_or_else(|| EK::JsonFieldMissing("Id"))
                        .map_err(Error::from_kind)?
                        .as_str()
                        .ok_or_else(|| EK::JsonTypeError("Id", "String"))
                        .map_err(Error::from_kind)?;

                    let post = &format!("/exec/{}/start", id);

                    self.docker
                        .stream_post(&post[..], Some(&mut bytes))
                        .map(|stream| Tty::new(stream))
                } else {
                    Err(Error::from_kind(EK::JsonTypeError("<anonymous>", "Object")))
                }
            }
        }
    }
*/
    /*
    /// Exec the specified command in the container
    pub fn exec(&self, opts: &ExecContainerOptions) -> Box<Stream<Item=String, Error=Error> + Send> {
        let path = Some(format!("/containers/{}/exec", self.id));
        let query: Option<String> = None;
        let body = opts.serialize().ok()
            .map(|a| str::as_bytes(&a))
            .map(Body::from);


        parse_to_trait::<Value>(self.docker.delete(path, query))
            .and_then(|value| {
                if let Value::Object(ref _obj) = value {
                    let id = value
                        .get("Id")
                        .ok_or_else(|| EK::JsonFieldMissing("Id"))
                        .map_err(Error::from_kind)?
                        .as_str()
                        .ok_or_else(|| EK::JsonTypeError("Id", "String"))
                        .map_err(Error::from_kind)?;

                    let path = Some(format!("/exec/{}/start", id));
                    let query : Option<String> = None;
                    let body : Option<Body> = None;

                    Ok(tty(self.docker.post(path, query, body)))
                } else {
                    Err(Error::from_kind(EK::JsonTypeError("<anonymous>", "Object")))
                }
            })
    }
    */
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