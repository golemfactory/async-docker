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
use communicate::util::RequestArgs;
use communicate::util::IntoRequestArgs;
use http::header::CONNECTION;
use http::header::HeaderValue;


pub(crate) trait InteractApi: Send + Sync
{
    fn request(&self, opts: RequestArgs, method: Method)
        -> ResponseFutureWrapper;
}

impl InteractApi for Arc<InteractApi>
{
    fn request(&self, opts: RequestArgs, method: Method)
               -> ResponseFutureWrapper
    {
        (**self).request(opts, method)
    }
}

pub(crate) trait InteractApiExt
{
    fn get<'a, 'b, A>(&self, opts: A) -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>;

    fn put<'a, 'b, A>(&self, opts: A)
           -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>;

    fn post<'a, 'b, A>(&self, opts: A)
            -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>;

    fn post_json<'a, 'b, A>(&self, opts: A)
                 -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>;

    fn delete<'a, 'b, A>(&self, opts: A) -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>;
}

impl <T> InteractApiExt for T
    where T: InteractApi
{
    fn get<'a, 'b, A>(&self, opts: A) -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>
    {
        self.request(opts.into_request_args(), Method::GET)
    }

    fn put<'a, 'b, A>(&self, opts: A) -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>
    {
        self.request(opts.into_request_args(), Method::PUT)
    }

    fn post<'a, 'b, A>(&self, opts: A) -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>
    {
        self.request(opts.into_request_args(), Method::POST)
    }

    fn post_json<'a, 'b, A>(&self, opts: A) -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>
    {
        let mut opts = opts.into_request_args();
        #[cfg(target_os = "linux")]
        opts.set_header(CONNECTION, HeaderValue::from_str("close")
            .expect("Constant connection header values's parse failed"));
        opts.set_header(CONTENT_TYPE, HeaderValue::from_str("application/json")
            .expect("Constant content type header value's parse failed"));

        self.request(opts, Method::POST)
    }

    fn delete<'a, 'b, A>(&self, opts: A) -> ResponseFutureWrapper
        where A: IntoRequestArgs<'a, 'b>
    {
        self.request(opts.into_request_args(), Method::DELETE)
    }
}


#[derive(Clone)]
pub(crate) struct Interact<I>
    where
        I: Connect + 'static
{
    client: Client<I>,
    host: Uri,
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
}

impl <I> InteractApi for Interact<I>
    where
        I: Connect + 'static
{
    fn request(&self, opts: RequestArgs, method: Method) -> ResponseFutureWrapper
    {
        let client = self.client.clone();
        let uri_result = compose_uri(&self.host, opts.path, opts.query);

        let b = opts.body;
        let h = opts.header;

        Box::new(future::result(uri_result)
            .and_then(move |uri|
                ::transport::build_request(method, uri, b)
                    .map_err(Error::from)
            )
            .map_err(Error::from)
            // Inserting header elements one-by-one
            .and_then(move |mut request| {
                for h in h {
                    let key = h.0.expect("Empty header's key name");
                    request.headers_mut().insert(key, h.1);
                }
                Ok(request)
            })
            .and_then( move |mut request| {
                Ok(client.request(request))
            })
        )
    }
}