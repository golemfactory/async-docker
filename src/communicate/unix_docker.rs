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


use transport::interact::Interact;
use docker::Docker;
use errors::Result;
use std::sync::Arc;
use communicate::docker::DockerApi;
use std::marker::PhantomData;

pub struct UnixConnector
{
    handle: Handle,
    path: PathBuf
}

impl UnixConnector {
    pub(crate) fn new(handle: Handle, path: PathBuf) -> Self
    {
        Self {
            handle,
            path
        }
    }
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
        let unix = UnixStream::connect(&self.path.as_path());

        let join = unix.join(connected);

        Box::new(join)
    }
}

pub(crate) type UnixDocker = Docker<UnixConnector>;

impl Docker<UnixConnector> {
    pub(crate) fn new(host: Uri) -> Result<Box<DockerApi>>
    {
        let path = format!("/{}{}", host.authority().unwrap(), host.path());
        let mut parts = host.into_parts();
        parts.authority = Some(Authority::from_str("v1.37").unwrap());
        parts.scheme = Some(Scheme::from_str("http").unwrap());
        let host = Uri::from_parts(parts).unwrap();
        let interact = Interact::new(
            Client::builder().build(
                UnixConnector::new(Handle::current(), PathBuf::from(path))
            ),
            host
        );

        let docker = Self::new_inner(Arc::new(interact));
        Ok(Box::new(docker))
    }
}
