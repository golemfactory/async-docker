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

pub fn build_simple_query<A>(name: &str, value: Option<A>) -> Option<String>
    where
        A: AsRef<str>
{
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