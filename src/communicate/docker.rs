use futures::Future;
use hyper::{client::connect::Connect, rt::Stream};

use errors::{Error, ErrorKind, Result};

use communicate::util::AsSlice;
use transport::{parse_to_stream, parse_to_trait};

use std::env;

use build::EventsOptions;

#[cfg(feature = "ssl")]
use super::ssl_tcp_docker::TcpSslDocker;
use super::tcp_docker::TcpDocker;
#[cfg(unix)]
use super::unix_docker::UnixDocker;
use communicate::{
    containers::Containers, image::Image, networks::Networks, Container, Images, Network,
};
use hyper::Uri;
use models::{SystemEventsResponse, SystemInfo, SystemVersionResponse};
use std::{borrow::Cow, marker::PhantomData, sync::Arc};
use transport::interact::{InteractApi, InteractApiExt};

/// Entry point interface for communicating with docker daemon
pub trait DockerApi: Send + Sync {
    /// Returns version information associated with the docker daemon
    fn version(&self) -> Box<Future<Item = SystemVersionResponse, Error = Error> + Send>;

    /// Returns information associated with the docker daemon
    fn info(&self) -> Box<Future<Item = SystemInfo, Error = Error> + Send>;

    /// Returns a simple ping response indicating the docker daemon is accessible
    fn ping(&self) -> Box<Future<Item = (), Error = Error> + Send>;

    /// Returns an iterator over streamed docker events
    fn events(
        &self,
        opts: &EventsOptions,
    ) -> Box<Stream<Item = SystemEventsResponse, Error = Error> + Send>;

    /// Exports an interface for interacting with docker container
    fn container(&self, id: Cow<'static, str>) -> Container;

    /// Exports an interface for interacting with docker containers
    fn containers(&self) -> Containers;

    /// Exports an interface for interacting with docker image
    fn image<'a>(&self, id: Cow<'a, str>) -> Image<'a>;

    /// Exports an interface for interacting with images
    fn images(&self) -> Images;

    /// Exports an interface for interacting with network
    fn network<'a>(&self, id: Cow<'a, str>) -> Network<'a>;

    /// Exports an interface for interacting with networks
    fn networks(&self) -> Networks;
}

pub(crate) struct Docker<C>
where
    C: Connect + 'static,
{
    interact: Arc<InteractApi>,
    phantom: PhantomData<C>,
}

impl<C> Docker<C>
where
    C: Connect + 'static,
{
    pub(super) fn new_inner(interact: Arc<InteractApi>) -> Self {
        Self {
            interact,
            phantom: PhantomData,
        }
    }
}

impl<C> DockerApi for Docker<C>
where
    C: Connect + 'static,
{
    fn version(&self) -> Box<Future<Item = SystemVersionResponse, Error = Error> + Send> {
        let arg = "/version";

        Box::new(parse_to_trait::<SystemVersionResponse>(
            self.interact.get(arg),
        ))
    }

    fn info(&self) -> Box<Future<Item = SystemInfo, Error = Error> + Send> {
        let arg = "/info";

        Box::new(parse_to_trait::<SystemInfo>(self.interact.get(arg)))
    }

    fn ping(&self) -> Box<Future<Item = (), Error = Error> + Send> {
        let arg = "/_ping";

        Box::new(parse_to_trait(self.interact.get(arg)))
    }

    fn events(
        &self,
        opts: &EventsOptions,
    ) -> Box<Stream<Item = SystemEventsResponse, Error = Error> + Send> {
        let query = opts.serialize();
        let arg = ("/events", query.as_slice());

        Box::new(parse_to_stream(self.interact.get(arg)))
    }

    fn container(&self, id: Cow<'static, str>) -> Container {
        let interact = self.interact.clone();
        Container::new(interact, id)
    }

    fn containers(&self) -> Containers {
        let interact = self.interact.clone();
        Containers::new(interact)
    }

    fn image<'a>(&self, id: Cow<'a, str>) -> Image<'a> {
        let interact = self.interact.clone();
        Image::new(interact, id)
    }

    fn images(&self) -> Images {
        let interact = self.interact.clone();
        Images::new(interact)
    }

    fn network<'a>(&self, id: Cow<'a, str>) -> Network<'a> {
        let interact = self.interact.clone();
        Network::new(interact, id)
    }

    fn networks(&self) -> Networks {
        let interact = self.interact.clone();
        Networks::new(interact)
    }
}

fn default_uri(uri: Option<Uri>) -> Result<Uri> {
    use communicate::util::{DEFAULT_URI, URI_ENV};
    match uri {
        None => match env::var(URI_ENV) {
            Ok(var) => var.parse().map_err(Error::from),
            Err(_) => DEFAULT_URI.parse().map_err(Error::from),
        },
        Some(x) => Ok(x),
    }
}

/// Creates the docker struct relevant to the provided Uri
pub fn new_docker(host: Option<Uri>) -> Result<Box<DockerApi>> {
    let host = default_uri(host)?;
    let scheme = host.scheme_part().map(|a| a.as_str().to_string());
    match scheme.as_slice() {
        Some(scheme) => match scheme {
            #[cfg(unix)]
            "unix" => UnixDocker::new(host),
            #[cfg(feature = "ssl")]
            "https" | "tcp" => TcpSslDocker::new(host),
            "http" => TcpDocker::new(host),
            _ => Err(ErrorKind::InvalidScheme.into()),
        },
        None => Err(ErrorKind::EmptyScheme.into()),
    }
}
