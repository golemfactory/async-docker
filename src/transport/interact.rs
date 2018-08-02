use hyper::Uri;
use std::sync::Arc;
use hyper::Client;
use transport::parse::ResponseFutureWrapper;
use Error;
use futures::future;
use hyper::Body;
use std::fmt::Display;
use hyper::Method;
use transport::parse::compose_uri;
use hyper::client::connect::Connect;
use futures::Future;


#[derive(Clone)]
pub struct Interact<I>
    where
        I: Connect + 'static
{
    pub(crate) client: Client<I>,
    pub(crate) host: Uri,
}

impl <I> Interact<I>
    where
        I: Connect + 'static
{
    pub fn new(client: Client<I>, host: Uri) -> Self {
        Interact {
            client,
            host
        }
    }

    fn request<A, B, C>(&self, path: Option<A>, query: Option<B>, body: Option<C>, method: Method)
                        -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default,
            C: Into<Body>,
    {
        let client = self.client.clone();
        let body = match body {
            None => Body::empty(),
            Some(a) => a.into(),
        };

        Box::new(future::result(compose_uri(&self.host, path, query))
            .and_then(|uri|
                ::transport::build_request(method, uri, Body::empty())
                    .map_err(Error::from)
            )
            .map_err(Error::from)
            .and_then( move |request|
                Ok(client.request(request)))
        )
    }

    pub fn get<A, B>(&self, path: Option<A>, query: Option<B>) -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default
    {
        let method = Method::GET;
        let body : Option<Body> = None;

        self.request(path, query, body, method)
    }

    pub fn post<A, B, C>(&self, path: Option<A>, query: Option<B>, body: Option<C>)
                     -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default,
            C: Into<Body>,
    {
        let method = Method::POST;

        self.request(path, query, body, method)
    }

    pub fn delete<A, B>(&self, path: Option<A>, query: Option<B>) -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default
    {
        let method = Method::DELETE;
        let body : Option<Body> = None;

        self.request(path, query, body, method)
    }
}