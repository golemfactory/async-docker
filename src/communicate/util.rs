pub use build::*;

pub use std::marker::Sized;
pub use errors::Error;
use errors::ErrorKind as EK;
/// Represents the result of all docker operations
pub use errors::Result;

use hyper::Body;
use hyper::body::Payload;
use hyper::Method;
use hyper::{Client, Uri};
use futures::Stream;

use tarball;

use representation::rep::{
    Change, Container as ContainerRep, ContainerCreateInfo, ContainerDetails, Exit, History,
    Image as ImageRep, ImageDetails, Info, NetworkCreateInfo, NetworkDetails as NetworkInfo,
    SearchResult, Status, Top, Version, Stats,
};

use std::str;
use std::borrow::Cow;
use std::env;
use std::io::Read;
use std::time::Duration;
use url::form_urlencoded;
use build::ContainerArchivePutOptions;
use std::io::Cursor;
use hyper::client::ResponseFuture;
use hyper::Request;
use hyper::rt::Future;
use hyper::client::connect::Connect;
use hyper::client::HttpConnector;
use http::uri::Parts;
use http::uri::PathAndQuery;
use std::str::FromStr;
use http::uri;
use hyper::Error as HyperError;

use std::io;
use hyper::client::connect::Destination;
use hyper::client::connect::Connected;
use http::uri::Scheme;
use tokio::reactor::Handle;
use std::net::IpAddr;
use futures::future::FutureResult;
use futures::future;
use futures::Join;
use std::io::Sink;
use std::path::Path;
use std::path::PathBuf;
use http::uri::Authority;
use serde::Serialize;
use serde::Deserialize;
use hyper::Chunk;
use hyper::Response;
use http::StatusCode;
use std::fmt::Display;
use std::fmt::Debug;
use std::result::IntoIter;
use representation::rep::Event;
use futures::stream::StreamFuture;
use futures::stream::Map;
use futures::stream;
use futures;

use transport::*;
use hyper::HeaderMap;
use http::header::HeaderValue;
use http::header::IntoHeaderName;
use http::header::HeaderName;


pub(crate) const URI_ENV: &'static str = "SHIPLIFT_URI";
pub(crate) const DEFAULT_URI: &'static str = "unix://var/run/docker.sock";

pub(crate) fn build_simple_query<A>(name: &str, value: Option<A>) -> Option<String>
    where
        A: AsRef<str>
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

impl AsSlice for Option<String>
{
    fn as_slice(&self) -> Option<&str> {
        match self {
            Some(ref x) => Some(x),
            None => None,
        }
    }
}

#[derive(Default)]
pub(crate) struct RequestArgs<'a,'b> {
    pub path: &'a str,
    pub query: &'b str,
    pub body: Body,
    pub header: HeaderMap,
}

impl <'a,'b> RequestArgs<'a,'b>{
    pub fn set_header<A,B>(&mut self, key: A, value: B) -> Result<()>
        where
            A: Into<HeaderName>,
            B: Into<HeaderValue>,
    {
        self.header.insert(key.into(), value.into());

        Ok(())
    }
}

pub(crate) trait IntoRequestArgs<'a,'b>
{
    fn into_request_args(self) -> RequestArgs<'a,'b>;
}

impl <'a,'b> IntoRequestArgs<'a,'b> for &'a str
{
    fn into_request_args(self) -> RequestArgs<'a,'b> {
        let mut args = RequestArgs::default();
        args.path = self;

        args
    }
}

impl <'a,'b> IntoRequestArgs<'a,'b> for (&'a str, Option<&'b str>)
{
    fn into_request_args(self) -> RequestArgs<'a,'b> {
        let mut args = RequestArgs::default();

        args.path = self.0;
        args.query = self.1.unwrap_or_default();

        args
    }
}

impl <'a,'b> IntoRequestArgs<'a,'b> for (&'a str, Option<Body>)
{
    fn into_request_args(self) -> RequestArgs<'a,'b> {
        let mut args = RequestArgs::default();
        args.path = self.0;
        args.body = self.1.unwrap_or_default();

        args
    }
}

impl <'a,'b> IntoRequestArgs<'a,'b> for (&'a str, Option<&'b str>, Option<Body>)
{
    fn into_request_args(self) -> RequestArgs<'a,'b> {
        let mut args = RequestArgs::default();

        args.path = self.0;
        args.query = self.1.unwrap_or_default();
        args.body = self.2.unwrap_or_default();

        args
    }
}