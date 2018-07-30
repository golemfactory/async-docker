#![cfg(target_os = "linux")]

extern crate tokio_uds;
extern crate unix_socket;
use self::tokio_uds::UnixStream;

use tokio::reactor::Handle;
use std::path::PathBuf;
use hyper::client::connect::Connect;
use tokio::io;
use tokio::prelude::Future;
use hyper::client::connect::Connected;
use hyper::client::connect::Destination;
use tokio::prelude::future;
use hyper::Uri;
use hyper::Client;
use http::uri::Authority;
use http::uri::Scheme;
use std::str::FromStr;


use docker::Docker;
use docker::DockerTrait;
use errors::Result;

const UNIX_SCHEME: &str = "unix";

pub struct UnixConnector {
    handle: Handle,
    path: PathBuf
}

impl Connect for UnixConnector {
    type Transport = UnixStream;
    type Error = io::Error;
    type Future = Box<Future<Item=(UnixStream, Connected), Error=io::Error> + Send>;

    fn connect(&self, dst: Destination) -> Self::Future {
        if dst.scheme() != "http" {
            return Box::new(future::err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid uri {:?}", dst),
            )));
        }

        let connected = future::ok(Connected::new());
        UnixStream::connect(&self.path.as_path());

        let unix = UnixStream::connect(&self.path.as_path());
        let join = unix.join(connected);

        Box::new(join)
    }
}

pub type UnixDocker = Docker<UnixConnector>;

impl DockerTrait for Docker<UnixConnector> {
    type Connector = UnixConnector;

    fn new(host: Uri) -> Result<Self> {
        let path = format!("/{}{}", host.authority().unwrap(), host.path());
        let mut parts = host.clone().into_parts();
        parts.authority = Some(Authority::from_str("v1.37").unwrap());
        parts.scheme = Some(Scheme::from_str("http").unwrap());
        let host = Uri::from_parts(parts).unwrap();

        Ok(UnixDocker {
            client: Client::builder().build(
                UnixConnector {
                    handle: Handle::current(),
                    path: PathBuf::from(path),
                }),
            host
        })
    }

    fn host(&self) -> &Uri {
        &self.host
    }

    fn client(&self) -> &Client<Self::Connector> {
        &self.client
    }
}