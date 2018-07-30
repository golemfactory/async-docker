pub use build::*;

pub use std::marker::Sized;
pub use errors::Error;
use errors::ErrorKind as EK;
/// Represents the result of all docker operations
pub use errors::Result;

use hyper::Body;
use hyper::body::Payload;
use hyper::Method;
use hyper::{Client, Uri};
use futures::Stream;

use tarball;

use representation::rep::{
    Change, Container as ContainerRep, ContainerCreateInfo, ContainerDetails, Exit, History,
    Image as ImageRep, ImageDetails, Info, NetworkCreateInfo, NetworkDetails as NetworkInfo,
    SearchResult, Status, Top, Version, Stats,
};

use std::str;
use std::borrow::Cow;
use std::env;
use std::io::Read;
use std::time::Duration;
use url::form_urlencoded;
use build::ContainerArchiveOptions;
use std::io::Cursor;
use hyper::client::ResponseFuture;
use hyper::Request;
use hyper::rt::Future;
use hyper::client::connect::Connect;
use hyper::client::HttpConnector;
use http::uri::Parts;
use http::uri::PathAndQuery;
use std::str::FromStr;
use http::uri;
use hyper::Error as HyperError;

use std::io;
use hyper::client::connect::Destination;
use hyper::client::connect::Connected;
use http::uri::Scheme;
use tokio::reactor::Handle;
use std::net::IpAddr;
use futures::future::FutureResult;
use futures::future;
use futures::Join;
use std::io::Sink;
use std::path::Path;
use std::path::PathBuf;
use http::uri::Authority;
use serde::Serialize;
use serde::Deserialize;
use hyper::Chunk;
use hyper::Response;
use http::StatusCode;
use std::fmt::Display;
use std::fmt::Debug;
use std::result::IntoIter;
use representation::rep::Event;
use futures::stream::StreamFuture;
use futures::stream::Map;
use futures::stream;
use futures;

use transport::*;
use ::docker::DockerTrait;

fn build_simple_query(name: &str, value: Option<&str>) -> Option<String> {
    let mut query = None;

    if let Some(ref val) = value {
        query = Some(form_urlencoded::serialize(vec![(name, val)]))
    };

    query
}

/*
/// Interface for accessing and manipulating a named docker image
pub struct Image<'a, 'b, T: 'a> {
    docker: &'a Docker<T>,
    name: Cow<'b, str>,
}

impl<'a, 'b, T> Image<'a, 'b, T> {
    /// Exports an interface for operations that may be performed against a named image
    pub fn new<S>(docker: &'a Docker<T>, name: S) -> Image<'a, 'b, T>
    where
        S: Into<Cow<'b, str>>,
    {
        Image {
            docker,
            name: name.into(),
        }
    }

    /// Inspects a named image's details
    pub fn inspect(&self) -> Result<ImageDetails> {
        let raw = self.docker.get(&format!("/images/{}/json", self.name)[..])?;
        ::serde_json::from_str::<ImageDetails>(&raw).map_err(Error::from)
    }

    /// Lists the history of the images set of changes
    pub fn history(&self) -> Result<Vec<History>> {
        let raw = self
            .docker
            .get(&format!("/images/{}/history", self.name)[..])?;
        ::serde_json::from_str::<Vec<History>>(&raw).map_err(Error::from)
    }

    /// Deletes an image
    pub fn delete(&self) -> Result<Vec<Status>> {
        let raw = self.docker.delete(&format!("/images/{}", self.name)[..])?;
        match ::serde_json::from_str(&raw)? {
            Value::Array(ref xs) => xs
                .iter()
                .map(|j| {
                    let obj = j
                        .as_object()
                        .ok_or_else(|| EK::JsonTypeError("<anonym>", "Object"))?;

                    if let Some(sha) = obj.get("Untagged") {
                        sha.as_str()
                            .map(|s| Status::Untagged(s.to_owned()))
                            .ok_or_else(|| EK::JsonTypeError("Untagged", "String"))
                    } else {
                        obj.get("Deleted")
                            .ok_or_else(|| EK::JsonFieldMissing("Deleted' or 'Untagged"))
                            .and_then(|sha| {
                                sha.as_str()
                                    .map(|s| Status::Deleted(s.to_owned()))
                                    .ok_or_else(|| EK::JsonTypeError("Deleted", "String"))
                            })
                    }
                })
                .map(|r| r.map_err(Error::from_kind)),

            _ => unreachable!(),
        }.collect()
    }

    /// Export this image to a tarball
    pub fn export(&self) -> Box<ResponseFuture> {
        self.docker
            .stream_get(&format!("/images/{}/get", self.name)[..])
    }
}
*/

/*
/// Interface for docker images
pub struct Images<'a, T: 'a> {
    docker: &'a Docker<T>,
}

impl<'a, T> Images<'a, T> {
    /// Exports an interface for interacting with docker images
    pub fn new(docker: &'a Docker<T>) -> Images<'a, T> {
        Images { docker }
    }

    /// Builds a new image build by reading a Dockerfile in a target directory
    pub fn build(&self, opts: &BuildOptions) -> Result<Vec<Value>> {
        let mut path = vec!["/build".to_owned()];

        if let Some(query) = opts.serialize() {
            path.push(query);
        }

        let mut bytes = vec![];

        tarball::dir(&mut bytes, &opts.path[..])?;

        let body = Body::BufBody(&bytes[..], bytes.len());

        self.docker
            .stream_post(&path.join("?"), Some(body))
            .and_then(|r| ::serde_json::from_reader::<_, Vec<_>>(r).map_err(Error::from))
    }

    /// Lists the docker images on the current docker host
    pub fn list(&self, opts: &ImageListOptions) -> Result<Vec<ImageRep>> {
        let mut path = vec!["/images/json".to_owned()];

        if let Some(query) = opts.serialize() {
            path.push(query);
        }

        let raw = self.docker.get(&path.join("?"))?;
        ::serde_json::from_str::<Vec<ImageRep>>(&raw).map_err(Error::from)
    }

    /// Returns a reference to a set of operations available for a named image
    pub fn get(&'a self, name: &'a str) -> Image<T> {
        Image::new(self.docker, name)
    }

    /// Search for docker images by term
    pub fn search(&self, term: &str) -> Result<Vec<SearchResult>> {
        let query = form_urlencoded::serialize(vec![("term", term)]);
        let raw = self.docker.get(&format!("/images/search?{}", query)[..])?;

        ::serde_json::from_str::<Vec<SearchResult>>(&raw).map_err(Error::from)
    }

    /// Pull and create a new docker images from an existing image
    pub fn pull(&self, opts: &PullOptions) -> Box<ResponseFuture> {
        let mut path = vec!["/images/create".to_owned()];

        if let Some(query) = opts.serialize() {
            path.push(query);
        }

        self.docker
            .bufreader_post(&path.join("?"), None as Option<&'a str>)
    }

    /// exports a collection of named images,
    /// either by name, name:tag, or image id, into a tarball
    pub fn export(&self, names: Vec<&str>) -> Box<ResponseFuture> {
        let params = names
            .iter()
            .map(|n| ("names", *n))
            .collect::<Vec<(&str, &str)>>();

        let query = form_urlencoded::serialize(params);

        self.docker
            .stream_get(&format!("/images/get?{}", query)[..])
    }

    // pub fn import(self, tarball: Box<Read>) -> Result<()> {
    //  self.docker.post
    // }
}
*/

/// Interface for accessing and manipulating a docker container
pub struct Container<'b, D, T>
    where
        D: DockerTrait<Connector=T>,
        T: 'static + Connect,
{
    docker: D,
    id: Cow<'b, str>,
}

impl<'a, 'b, D, T> Container<'b, D, T>
    where
        D: DockerTrait<Connector=T>,
        T: 'static + Connect,
{
    /// Exports an interface exposing operations against a container instance
    pub fn new<S>(docker: D, id: S) -> Container<'b, D, T>
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
        let string = format!("/containers/{}/json", self.id);
        let path = Some(string.as_ref());
        let query : Option<&str> = None;

        Box::new(parse_to_trait::<ContainerDetails>(self.docker.get(path, query)))
    }

    /// Returns a `top` view of information about the container process
    pub fn top(&self, psargs: Option<&str>) -> Box<Future<Item=Top, Error=Error> + Send> {
        let string = format!("/containers/{}/top", self.id);
        let path = Some(string.as_ref());
        let query = build_simple_query("ps_args", psargs);

        Box::new(parse_to_trait::<Top>(self.docker.get(path, query)))
    }

    /// Returns a stream of logs emitted but the container instance
    pub fn logs(&self, opts: &LogsOptions) -> Box<Stream<Item=String, Error=Error> + Send> {
        let string = format!("/containers/{}/logs", self.id);
        let path = Some(string.as_ref());
        let query = opts.serialize();

        Box::new(parse_to_lines(self.docker.get(path, query)))
    }

    /// Returns a set of changes made to the container instance
    pub fn changes(&self) -> Box<Future<Item=Vec<Change>, Error=Error> + Send> {
        let string = format!("/containers/{}/changes", self.id);
        let path = Some(string.as_ref());
        let query : Option<&str>  = None;

        Box::new(parse_to_trait::<Vec<Change>>(self.docker.get(path, query)))
    }
/*
    /// Exports the current docker container into a tarball
    pub fn export(&self) -> Box<ResponseFuture> {
        self.docker
            .stream_get(&format!("/containers/{}/export", self.id)[..])
    }
*/
    /// Returns a stream of stats specific to this container instance
    pub fn stats(&self) -> Box<Stream<Item=Result<Stats>, Error=Error> + Send> {
        let string = format!("/containers/{}/stats", self.id);
        let path = Some(string.as_ref());
        let query : Option<&str> = None;

        Box::new(parse_to_stream::<Stats>(self.docker.get(path, query)))
    }

    /*
    /// Start the container instance
    pub fn start(&'a self) -> Result<()> {
        let s = &format!("/containers/{}/start", self.id)[..];
        self.docker
            .post(s, None as Option<&'a str>)
            .map(|_| ())
    }

    /// Stop the container instance
    pub fn stop(&self, wait: Option<Duration>) -> Result<()> {
        let mut path = vec![format!("/containers/{}/stop", self.id)];

        if let Some(w) = wait {
            let encoded = form_urlencoded::serialize(vec![("t", w.as_secs().to_string())]);
            path.push(encoded);
        }

        self.docker
            .post(&path.join("?"), None as Option<&'a str>)
            .map(|_| ())
    }

    /// Restart the container instance
    pub fn restart(&self, wait: Option<Duration>) -> Result<()> {
        let mut path = vec![format!("/containers/{}/restart", self.id)];

        if let Some(w) = wait {
            let encoded = form_urlencoded::serialize(vec![("t", w.as_secs().to_string())]);

            path.push(encoded);
        }

        self.docker
            .post(&path.join("?"), None as Option<&'a str>)
            .map(|_| ())
    }

    /// Kill the container instance
    pub fn kill(&self, signal: Option<&str>) -> Result<()> {
        let mut path = vec![format!("/containers/{}/kill", self.id)];

        if let Some(sig) = signal {
            let encoded = form_urlencoded::serialize(vec![("signal", sig.to_owned())]);
            path.push(encoded)
        }

        self.docker
            .post(&path.join("?"), None as Option<&'a str>)
            .map(|_| ())
    }

    /// Rename the container instance
    pub fn rename(&self, name: &str) -> Result<()> {
        let query = form_urlencoded::serialize(vec![("name", name)]);
        let s = &format!("/containers/{}/rename?{}", self.id, query)[..];

        self.docker
            .post(s, None as Option<&'a str>)
            .map(|_| ())
    }

    /// Pause the container instance
    pub fn pause(&self) -> Result<()> {
        let s = &format!("/containers/{}/pause", self.id)[..];

        self.docker
            .post(s, None as Option<&'a str>)
            .map(|_| ())
    }

    /// Unpause the container instance
    pub fn unpause(&self) -> Result<()> {
        let s = &format!("/containers/{}/unpause", self.id)[..];

        self.docker
            .post(s, None as Option<&'a str>)
            .map(|_| ())
    }

    /// Wait until the container stops
    pub fn wait(&self) -> Result<Exit> {
        let s = &format!("/containers/{}/wait", self.id)[..];
        let raw = self.docker.post(s, None as Option<&'a str>)?;

        ::serde_json::from_str::<Exit>(&raw).map_err(Error::from)
    }

    /// Delete the container instance
    ///
    /// Use remove instead to use the force/v options.
    pub fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/containers/{}", self.id)[..])
            .map(|_| ())
    }

    /// Delete the container instance (todo: force/v)
    pub fn remove(&self, opts: RmContainerOptions) -> Result<()> {
        let mut path = vec![format!("/containers/{}", self.id)];

        if let Some(query) = opts.serialize() {
            path.push(query);
        }

        self.docker.delete(&path.join("?"))?;
        Ok(())
    }

    /// Exec the specified command in the container
    pub fn exec(&self, opts: &ExecContainerOptions) -> Result<Tty> {
        let data = opts.serialize()?;
        let mut bytes = data.as_bytes();

        let s = &format!("/containers/{}/exec", self.id)[..];

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


/*
/// Interface for docker containers
pub struct Containers<'a, T: 'a> {
    docker: &'a Docker<T>,
}

impl<'a, T> Containers<'a, T> {
    /// Exports an interface for interacting with docker containers
    pub fn new(docker: &'a Docker<T>) -> Containers<'a, T> {
        Containers { docker }
    }

    /// Lists the container instances on the docker host
    pub fn list(&self, opts: &ContainerListOptions) -> Result<Vec<ContainerRep>> {
        let mut path = vec!["/containers/json".to_owned()];

        if let Some(query) = opts.serialize() {
            path.push(query)
        }

        let raw = self.docker.get(&path.join("?"))?;
        ::serde_json::from_str::<Vec<ContainerRep>>(&raw).map_err(Error::from)
    }

    /// Returns a reference to a set of operations available to a specific container instance
    pub fn get(&'a self, name: &'a str) -> Container<T> {
        Container::new(self.docker, name)
    }

    /// Returns a builder interface for creating a new container instance
    pub fn create(&'a self, opts: &ContainerOptions) -> Result<ContainerCreateInfo> {
        let data = opts.serialize()?;
        let mut bytes = data.as_bytes();
        let mut path = vec!["/containers/create".to_owned()];

        if let Some(ref name) = opts.name {
            path.push(form_urlencoded::serialize(vec![("name", name)]));
        }

        let raw = self
            .docker
            .post(&path.join("?"), Some(&mut bytes))?;

        ::serde_json::from_str::<ContainerCreateInfo>(&raw).map_err(Error::from)
    }
}
*/

/*
/// Interface for docker network
pub struct Networks<'a, T: 'a> {
    docker: &'a Docker<T>,
}

impl<'a, T> Networks<'a, T> {
    /// Exports an interface for interacting with docker Networks
    pub fn new(docker: &'a Docker<T>) -> Networks<'a, T> {
        Networks { docker }
    }

    /// List the docker networks on the current docker host
    pub fn list(&self, opts: &NetworkListOptions)
            -> Box<Future<Item=Vec<NetworkInfo>, Error=Error>> {
        let mut path = vec!["/networks".to_owned()];

        if let Some(query) = opts.serialize() {
            path.push(query);
        }

        let res = self.docker
            .get(&path.join("?"))
            .and_then(|response |
                          response.into_body().concat2()
            )
            .and_then(|body | {
                let vec = body.iter().cloned().collect();
                let stringify = String::from_utf8(vec).map_err(Error::from)?;
                println!("{}", stringify);
                ::serde_json::from_str::<Vec<NetworkInfo>>(&stringify)
                    .map_err(Error::from)
            });

        Box::new(res)
    }

    /// Returns a reference to a set of operations available to a specific network instance
    pub fn get(&'a self, id: &'a str) -> Network<T> {
        Network::new(self.docker, id)
    }

    pub fn create(&'a self, opts: &NetworkCreateOptions)
            -> Box<Future<Item=NetworkCreateInfo, Error=Error>> {
        let data = opts.serialize();
        let mut bytes = data.as_bytes();
        let path = vec!["/networks/create".to_owned()];

        self.docker.post(&path.join("?"), &mut bytes)
            .and_then(|response|
                ::serde_json::from_reader::<NetworkCreateInfo>(
                    response.into_body().poll_data()).map_err(Error::from))
    }
}

/// Interface for accessing and manipulating a docker network
pub struct Network<'a, 'b, T: 'a> {
    docker: &'a Docker<T>,
    id: Cow<'b, str>,
}

impl<'a, 'b, T> Network<'a, 'b, T> {
    /// Exports an interface exposing operations against a network instance
    pub fn new<S>(docker: &'a Docker<T>, id: S) -> Network<'a, 'b, T>
    where
        S: Into<Cow<'b, str>>,
    {
        Network {
            docker,
            id: id.into(),
        }
    }

    /// a getter for the Network id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Inspects the current docker network instance's details
    pub fn inspect(&self) -> Box<Future<Item=NetworkInfo, Error=Error>> {
        self.docker.get(&format!("/networks/{}", self.id)[..])?
            .and_then(|response|
                ::serde_json::from_str::<NetworkInfo>(response?).map_err(Error::from))
    }

    /// Delete the network instance
    pub fn delete(&self) -> Box<Future<Item=(), Error=Error>> {
        self.docker.delete(&format!("/networks/{}", self.id)[..])?
            .and_then(|_| () )
    }

    /// Connect container to network
    pub fn connect(&self, opts: &ContainerConnectionOptions)
            -> Box<Future<Item=(), Error=Error>> {
        self.do_connection("connect", opts)
    }

    /// Disconnect container to network
    pub fn disconnect(&self, opts: &ContainerConnectionOptions)
            -> Box<Future<Item=(), Error=Error>> {
        self.do_connection("disconnect", opts)
    }

    fn do_connection(&self, segment: &str, opts: &ContainerConnectionOptions)
            -> Box<Future<Item=(), Error=Error>> {
        let data = opts.serialize()?;
        let mut bytes = data.as_bytes();

        let s = &format!("/networks/{}/{}", self.id, segment)[..];

        self.get(s)?.and_then(|_| () )
    }
}
*/