pub use build::*;

pub use errors::Error;
/// Represents the result of all docker operations
pub use errors::Result;
pub use std::marker::Sized;

use http::header::{HeaderName, HeaderValue};
use hyper::{Body, HeaderMap};
use std::str;
use url::form_urlencoded;

pub(crate) const URI_ENV: &'static str = "DOCKER_HOST";
pub(crate) const DEFAULT_URI: &'static str = "unix://var/run/docker.sock";

pub(crate) fn build_simple_query<A>(name: &str, value: Option<A>) -> Option<String>
where
    A: AsRef<str>,
{
    let mut query = None;

    if let Some(ref val) = value {
        query = Some(form_urlencoded::serialize(vec![(name, val)]))
    };

    query
}

pub(crate) trait AsSlice {
    fn as_slice(&self) -> Option<&str>;
}

impl AsSlice for Option<String> {
    fn as_slice(&self) -> Option<&str> {
        match self {
            Some(ref x) => Some(x),
            None => None,
        }
    }
}

#[derive(Default)]
pub(crate) struct RequestArgs<'a, 'b> {
    pub path: &'a str,
    pub query: &'b str,
    pub body: Body,
    pub header: HeaderMap,
}

impl<'a, 'b> RequestArgs<'a, 'b> {
    pub fn set_header<A, B>(&mut self, key: A, value: B)
    where
        A: Into<HeaderName>,
        B: Into<HeaderValue>,
    {
        self.header.insert(key.into(), value.into());
    }
}

pub(crate) trait IntoRequestArgs<'a, 'b> {
    fn into_request_args(self) -> RequestArgs<'a, 'b>;
}

impl<'a, 'b> IntoRequestArgs<'a, 'b> for &'a str {
    fn into_request_args(self) -> RequestArgs<'a, 'b> {
        let mut args = RequestArgs::default();
        args.path = self;

        args
    }
}

impl<'a, 'b> IntoRequestArgs<'a, 'b> for (&'a str, Option<&'b str>) {
    fn into_request_args(self) -> RequestArgs<'a, 'b> {
        let mut args = RequestArgs::default();

        args.path = self.0;
        args.query = self.1.unwrap_or_default();

        args
    }
}

impl<'a, 'b> IntoRequestArgs<'a, 'b> for (&'a str, Option<Body>) {
    fn into_request_args(self) -> RequestArgs<'a, 'b> {
        let mut args = RequestArgs::default();
        args.path = self.0;
        args.body = self.1.unwrap_or_default();

        args
    }
}

impl<'a, 'b> IntoRequestArgs<'a, 'b> for (&'a str, Option<&'b str>, Option<Body>) {
    fn into_request_args(self) -> RequestArgs<'a, 'b> {
        let mut args = RequestArgs::default();

        args.path = self.0;
        args.query = self.1.unwrap_or_default();
        args.body = self.2.unwrap_or_default();

        args
    }
}
