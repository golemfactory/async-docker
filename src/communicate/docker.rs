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

pub struct Docker<C>
    where
        C: Connect + 'static
{
    pub(crate) client: Client<C>,
    pub(crate) host: Uri,
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

    fn host(&self) -> &Uri;

    fn client(&self) -> &Client<Self::Connector>;

    /*
    /// Exports an interface for interacting with docker images
    pub fn images<'a>(&'a self) -> Images<T> {
        Images::new(self)
    }

    /// Exports an interface for interacting with docker containers
    pub fn containers<'a>(&'a self) -> Containers<T> {
        Containers::new(self)
    }
    */

    /*
    pub fn networks<'a>(&'a self) -> Networks<T> {
        Networks::new(self)
    }
*/
    /// Returns version information associated with the docker daemon
    fn version(&self) -> Box<Future<Item=Version, Error=Error> + Send> {
        let path = Some("/version");
        let query : Option<String> = None;

        parse_to_trait::<Version>(self.get(path, query))
    }

    /// Returns information associated with the docker daemon
    fn info(&self) -> Box<Future<Item=Info, Error=Error> + Send> {
        let path = Some("/info");
        let query : Option<String> = None;

        parse_to_trait::<Info>(self.get(path, query))
    }

    /// Returns a simple ping response indicating the docker daemon is accessible
    fn ping(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let path = Some("/_ping");
        let query : Option<String> = None;

        status_code(self.get(path, query))
    }

    //noinspection Annotator
    //noinspection ALL
    /// Returns an iterator over streamed docker events
    fn events(&self, opts: &EventsOptions) -> Box<Stream<Item=Result<Event>, Error=Error> + Send> {
        let path = Some("/events");
        let query : Option<String> = opts.serialize();

        Box::new(parse_to_stream::<Event>(self.get(path, query)))
    }


    fn request<A, B, C>(&self, path: Option<A>, query: Option<B>, body: Option<C>, method: Method)
                     -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default,
            C: Into<Body>,
    {
        let client = self.client().clone();
        let body = match body {
            None => Body::empty(),
            Some(a) => a.into(),
        };

        Box::new(future::result(compose_uri(self.host(), path, query))
            .and_then(|uri|
                ::transport::build_request(method, uri, Body::empty())
                    .map_err(Error::from)
            )
            .map_err(Error::from)
            .and_then( move |request|
                Ok(client.request(request)))
        )
    }

    fn get<A, B>(&self, path: Option<A>, query: Option<B>) -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default
    {
        let method = Method::GET;
        let body : Option<Body> = None;

        self.request(path, query, body, method)
    }

    fn post<A, B, C>(&self, path: Option<A>, query: Option<B>, body: Option<C>)
                  -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default,
            C: Into<Body>,
    {
        let method = Method::POST;

        self.request(path, query, body, method)
    }

    fn delete<A, B>(&self, path: Option<A>, query: Option<B>) -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default
    {
        let method = Method::DELETE;
        let body : Option<Body> = None;

        self.request(path, query, body, method)
    }

    /*
    fn stream_put<'a, B>(
        &'a self,
        endpoint: &str,
        body: Option<B>,
    ) -> Box<ResponseFuture>
        where
            B: Into<Body>,
    {
        self.transport.build_response(Method::Put, endpoint, body)
    }

    fn stream_get<'a>(&self, endpoint: &str) -> Box<ResponseFuture> {
        self.transport.build_response(
            Method::Get,
            endpoint,
            None as Option<&'a str>,
        )
    }
*/
}
