//! Transports for communicating with the docker daemon

extern crate hyper;

//use hyper::buffer::BufReader;
use errors::{ErrorKind, Result};

//use super::reader::BufIterator;

use http;
use hyper::Body;
use hyper::Method;
use hyper::Client;
use hyper::Uri;
use hyper::client::connect::Connect;
use std::fmt;
use hyper::Request;
use transport::Transport::{Tcp, Unix};
use hyper::client::ResponseFuture;
use hyper::rt::Future;
use std::convert::Into;

use errors::ErrorKind as EK;
use hyper::client::HttpConnector;
use errors::Error;
use std::error::Error as StdError;

/// Transports are types which define the means of communication
/// with the docker daemon
pub enum Transport<T>
{
    /// A network tcp interface
    Tcp { client: Client<HttpConnector>, uri: Uri },
    /// A Unix domain socket
    Unix { client: Client<T>, uri: Uri },
}

impl <T: Connect> fmt::Debug for Transport<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tcp { ref uri, .. } => write!(f, "Tcp({})", uri),
            Unix { ref uri, .. } => write!(f, "Unix({})", uri),
        }
    }
}


/*
pub fn response<'a, B>(&'a self, method: Method, endpoint: &str, body: B)
    -> Result<String>
where
    B: Into<Body>,
{
    let mut res = self.stream(method, endpoint, body)?;
    let mut body = String::new();
    let _ = res.poll()?;

    debug!("{} raw response: {}", endpoint, body);
    Ok(body)
}
*/

pub fn build_request<B>(method: Method, uri: Uri, body: B)
    -> Result<Request<Body>>
    where
        B: Into<Body>,
{
    let body: Body = body.into();

    hyper::Request::builder()
        .method(method)
        .uri(uri)
        .body(body)
        .map_err(Error::from)
}

/*
pub fn bufreader<'c, B, T>(&'c self, method: Method, endpoint: &str, body: Option<B>)
    -> Result<super::reader::BufIterator<T>>
where
    B: Into<Body>,
    T: DeserializeOwned,
{
    let req = self.build_request(method, endpoint, body)?;

    Ok(BufIterator::<T>::new(res))
}*/



/*
/// Extract the error message content from an HTTP response that
/// contains a Docker JSON error structure.
#[allow(dead_code)]
fn get_error_message(res: &mut Response) -> Option<String> {
    let mut output = String::new();

    if res.read_to_string(&mut output).is_ok() {
        json::Json::from_str(output.as_str())
            .ok()
            .as_ref()
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("message"))
            .and_then(|x| x.as_string())
            .map(|x| x.to_owned())
    } else {
        None
    }
}
*/