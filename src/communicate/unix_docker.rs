#![cfg(unix)]

extern crate tokio_uds;
extern crate unix_socket;
use self::tokio_uds::UnixStream;

use http::uri::{Authority, Scheme};
use hyper::{
    client::connect::{Connect, Connected, Destination},
    Client, Uri,
};
use std::{path::PathBuf, str::FromStr};
use tokio::{
    io,
    prelude::{future, Future},
};

use communicate::docker::DockerApi;
use docker::Docker;
use errors::Result;
use std::sync::Arc;
use transport::interact::Interact;

pub struct UnixConnector {
    path: PathBuf,
}

impl UnixConnector {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Connect for UnixConnector {
    type Transport = UnixStream;
    type Error = io::Error;
    type Future = Box<Future<Item = (UnixStream, Connected), Error = io::Error> + Send>;

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
    pub(crate) fn new(host: Uri) -> Result<Box<DockerApi>> {
        let path = format!(
            "/{}{}",
            host.authority_part()
                .map(|a| a.as_str())
                .unwrap_or_default(),
            host.path()
        );
        let mut parts = host.into_parts();
        parts.authority =
            Some(Authority::from_str("v1.40").expect("Constant authority parsing error"));
        parts.scheme = Some(Scheme::from_str("http").expect("Constant scheme parsing error"));

        let host = Uri::from_parts(parts)?;
        let interact = Interact::new(
            Client::builder().build(UnixConnector::new(PathBuf::from(path))),
            host,
        );

        let docker = Self::new_inner(Arc::new(interact));
        Ok(Box::new(docker))
    }
}
