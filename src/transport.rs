//! Transports for communicating with the docker daemon

extern crate hyper;

//use hyper::buffer::BufReader;
use hyper::StatusCode;
use errors::{ErrorKind, Result};

//use super::reader::BufIterator;

use hyper::Response;
use hyper::Body;
use hyper::header;
use hyper::Method;
use hyper::Client;
use hyper::Uri;
use hyper::client::connect::Connect;
use serde::de::DeserializeOwned;
use std::fmt;
use std::io::Read;
use hyper::Request;
use hyper::HeaderMap;
use std::str::FromStr;
use transport::Transport::{Tcp, Unix};
use hyper::client::ResponseFuture;
use hyper::rt::Future;

/// Transports are types which define the means of communication
/// with the docker daemon
pub enum Transport<T>
{
    /// A network tcp interface
    Tcp { client: Client<T>, host: String },
    /// A Unix domain socket
    Unix { client: Client<T>, path: String },
}

impl <T: Connect> fmt::Debug for Transport<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tcp { ref host, .. } => write!(f, "Tcp({})", host),
            Unix { ref path, .. } => write!(f, "Unix({})", path),
        }
    }
}

impl <T: Connect> Transport<T> {
    pub fn client(&self) -> Client<T> {
        match *self {
            Tcp { ref client, .. } => *client,
            Unix { ref client, .. } => *client
        }
    }

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

    pub fn build_request<'c, B>(&'c self, method: Method, endpoint: &str, body: B)
        -> Result<Request<B>>
    {
        let (client, host) = match *self {
            Tcp {ref client, ref host} => (client, host),
            Unix {ref client, ref path} => (client, path)
        };

        let req = Request::builder()
            .method(method)
            .uri(endpoint)
            .body(body);
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

    pub fn stream<'c, B>(&'c self, method: Method, endpoint: &str, body: B)
        -> Result<ResponseFuture>
        where
        B: Into<Body>,
    {
        let body = match body {
            () => Body::empty(),
            a => Body::wrap_stream(Stream)
        };

        let mut req = self.build_request(method, endpoint, body);
        Ok(self.client().request(req))

        /*
        match res.status {
            StatusCode::Ok
            | StatusCode::Created
            | StatusCode::SwitchingProtocols => Ok(Box::new(res)),
            StatusCode::NoContent => Ok(Box::new(hyper::Client::new())),
            // todo: constantize these
            StatusCode::BadRequest
            | StatusCode::NotFound
            | StatusCode::NotAcceptable
            | StatusCode::Conflict
            | StatusCode::InternalServerError => Err(ErrorKind::HyperFault(res.status).into()),
            _ => unreachable!(),
        }*/
    }
}

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