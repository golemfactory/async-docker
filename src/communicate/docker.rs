use hyper::client::connect::Connect;
use hyper::Client;
use futures::Future;
use futures::future;
use std::fmt::Display;
use hyper::Body;
use hyper::rt::Stream;

use errors::{Result, Error};

use transport::status_code;
use transport::compose_uri;
use transport::parse_to_trait;
use transport::parse_to_stream;

use build::{
    EventsOptions,
};

use representation::rep::{
    Change, Container as ContainerRep, ContainerCreateInfo, ContainerDetails, Exit, History,
    Image as ImageRep, ImageDetails, Info, NetworkCreateInfo, NetworkDetails as NetworkInfo,
    SearchResult, Status, Top, Version, Stats, Event
};

use hyper::Uri;
use hyper::StatusCode;
use hyper::Method;
use transport::parse::ResponseFutureWrapper;
use transport::interact::Interact;
use std::sync::Arc;
use communicate::Container;
use std::borrow::Cow;

pub struct Docker<C>
    where
        C: Connect + 'static
{
    pub(crate) interact: Arc<Interact<C>>
}

/// Entry point interface for communicating with docker daemon
pub trait DockerTrait
    where
        Self: Sized + Sync + Send
{
    type Connector : Connect + 'static;

    /// constructs a new Docker instance for a docker host listening at a url specified by an env var `DOCKER_HOST`,
    /// falling back on unix:///var/run/docker.sock


    /// constructs a new Docker instance for docker host listening at the given host url
    fn new(host: Uri) -> Result<Self>;

    fn interact(&self) -> Arc<Interact<Self::Connector>>;

    /*
    /// Exports an interface for interacting with docker images
    pub fn images<'a>(&'a self) -> Images<T> {
        Images::new(self)
    }
    */
    /// Exports an interface for interacting with docker containers
    fn container<'b, S>(&self, id: S) -> Container<'b, Self::Connector>
        where
            S: Into<Cow<'b, str>>
    {
        let interact = self.interact().clone();
        Container::new(interact, id.into())
    }


    /*
    pub fn networks<'a>(&'a self) -> Networks<T> {
        Networks::new(self)
    }
*/
    /// Returns version information associated with the docker daemon
    fn version(&self) -> Box<Future<Item=Version, Error=Error> + Send> {
        let path = Some("/version");
        let query : Option<String> = None;

        Box::new(parse_to_trait::<Version>(self.interact().get(path, query)))
    }

    /// Returns information associated with the docker daemon
    fn info(&self) -> Box<Future<Item=Info, Error=Error> + Send> {
        let path = Some("/info");
        let query : Option<String> = None;

        Box::new(parse_to_trait::<Info>(self.interact().get(path, query)))
    }

    /// Returns a simple ping response indicating the docker daemon is accessible
    fn ping(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some("/_ping");
        let query : Option<String> = None;

        Box::new(status_code(self.interact().get(path, query)))
    }

    //noinspection Annotator
    //noinspection ALL
    /// Returns an iterator over streamed docker events
    fn events(&self, opts: &EventsOptions) -> Box<Stream<Item=Result<Event>, Error=Error> + Send> {
        let path = Some("/events");
        let query : Option<String> = opts.serialize();

        Box::new(parse_to_stream::<Event>(self.interact().get(path, query)))
    }
}
