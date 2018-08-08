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
use hyper::HeaderMap;
use http::header::CONTENT_TYPE;


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

    fn request_with_header<A, B, C>(&self, path: Option<A>, query: Option<B>,
                                    body: Option<C>, method: Method, header: Option<HeaderMap>)
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
        let mut header = match header {
            None => HeaderMap::new(),
            Some(h) => h,
        };


        Box::new(future::result(compose_uri(&self.host, path, query))
            .and_then(|uri|
                ::transport::build_request(method, uri, body)
                    .map_err(Error::from)
            )
            .map_err(Error::from)
            .and_then(|mut request| {
                for h in header {
                    let key = h.0.expect("Empty header key");
                    request.headers_mut().insert(key, h.1);
                }
                Ok(request)
            })
            .and_then( move |mut request| {
                Ok(client.request(request))
            })
        )
    }

    pub fn get<A, B>(&self, path: Option<A>, query: Option<B>) -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default
    {
        let method = Method::GET;
        let body : Option<Body> = None;

        self.request_with_header(path, query, body, method, None)
    }

    pub fn post<A, B>(&self, path: Option<A>, query: Option<B>)
                     -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default,
    {
        let method = Method::POST;
        let body : Option<Body> = None;

        self.request_with_header(path, query, body, method, None)
    }

    pub fn post_json<A, B, C>(&self, path: Option<A>, query: Option<B>, body: Option<C>)
                              -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default,
            C: Into<Body>,
    {
        let method = Method::POST;
        let mut map = HeaderMap::new();
        map.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let header = Some(map);

        self.request_with_header(path, query, body, method, header)
    }

    // Ugly workaround - there is issue with keep-alive chaining while connecting by Unix connector.
    // It just doesn't work.
    pub fn _post_json<A, B, C>(&self, path: Option<A>, query: Option<B>, body: Option<C>)
                              -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default,
            C: Into<Body>,
    {
        let method = Method::POST;
        let mut map = HeaderMap::new();
        map.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        map.insert("Connection", "close".parse().unwrap());
        let header = Some(map);

        self.request_with_header(path, query, body, method, header)
    }

    pub fn delete<A, B>(&self, path: Option<A>, query: Option<B>) -> ResponseFutureWrapper
        where
            A: AsRef<str> + Display + Default,
            B: AsRef<str> + Display + Default
    {
        let method = Method::DELETE;
        let body : Option<Body> = None;

        self.request_with_header(path, query, body, method, None)
    }
}