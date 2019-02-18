//! Transports for communicating with the docker daemon
extern crate tokio_codec;

use Result;

use hyper::{client::ResponseFuture, rt::Future, Body, Method, Request, Uri};
use std::convert::Into;

use self::tokio_codec::{BytesCodec, FramedWrite};
use super::lines::Lines;
use bytes::Bytes;
use errors::*;
use futures::{future, Sink, Stream};
use http::{uri::PathAndQuery, StatusCode};
use hyper::{Chunk, Response};
use models::ErrorResponse;
use serde_json::from_str as de_from_str;
use std::{
    fmt::Debug,
    path::Path,
    str::{self, FromStr},
};
use tokio::fs::File;

pub type ResponseFutureWrapper = Box<Future<Item = ResponseFuture, Error = Error> + Send>;

pub(crate) fn build_request<B>(method: Method, uri: Uri, body: B) -> Result<Request<Body>>
where
    B: Into<Body>,
{
    let body: Body = body.into();

    Request::builder()
        .method(method)
        .uri(uri)
        .body(body)
        .map_err(Error::from)
}

pub(crate) fn empty_result2(
    future: ResponseFutureWrapper,
) -> impl Future<Item = StatusCode, Error = Error> + Send {
    future.and_then(|w| {
        w.map_err(Error::from).and_then(|response| {
            let status = response.status();
            if status.is_success() {
                future::Either::A(future::ok(status))
            } else {
                future::Either::B(
                    response
                        .into_body()
                        .concat2()
                        .map_err(Error::from)
                        .and_then(move |chunk| {
                            let body = str::from_utf8(chunk.as_ref()).map_err(Error::from)?;

                            Err(match de_from_str::<ErrorResponse>(body) {
                                Ok(x) => ErrorKind::DockerApi(x, status).into(),
                                Err(_) => ErrorKind::DockerApiUnknown(body.to_string(), status),
                            }
                            .into())
                        })
                        .and_then(|_: StatusCode| Ok(StatusCode::default())),
                )
            }
        })
    })
}

pub(crate) fn empty_result(
    future: ResponseFutureWrapper,
) -> impl Future<Item = (), Error = Error> + Send {
    parse_to_trait::<()>(future).and_then(|_| Ok(()))
}

pub(crate) fn parse_to_trait<T>(
    future: ResponseFutureWrapper,
) -> impl Future<Item = T, Error = Error> + Send
where
    T: for<'a> ::serde::Deserialize<'a> + Send + 'static,
{
    future.and_then(|w| {
        w.map_err(Error::from).and_then(|response| {
            let status = response.status();
            response
                .into_body()
                .concat2()
                .map_err(Error::from)
                .and_then(move |chunk| parse_single_field(&chunk, status))
        })
    })
}

pub(crate) fn parse_to_lines(
    future: ResponseFutureWrapper,
) -> impl Stream<Item = String, Error = Error> + Send + 'static {
    future
        .and_then(|w| {
            w.map_err(Error::from)
                .and_then(|response| {
                    let body = transform_stream(response);

                    let lines = Lines::new(body);

                    Ok(lines)
                })
                .map_err(Error::from)
        })
        .flatten_stream()
}

pub(crate) fn parse_to_stream<T>(
    future: ResponseFutureWrapper,
) -> impl Stream<Item = T, Error = Error> + Send + 'static
where
    T: for<'a> ::serde::Deserialize<'a> + Send + Debug + 'static,
{
    future
        .and_then(|w| {
            w.map_err(Error::from)
                .and_then(|response| {
                    let status = response.status();
                    let body = transform_stream(response);

                    let lines = Lines::new(body);

                    let mapped =
                        lines.map(move |chunk| parse_single_field(&Chunk::from(chunk), status));

                    Ok(mapped)
                })
                .map_err(Error::from)
        })
        .flatten_stream()
        .and_then(|res| res)
}

#[allow(dead_code)]
pub(crate) fn parse_to_file(
    future: ResponseFutureWrapper,
    filepath: &'static str,
) -> impl Future<Item = (), Error = Error> {
    let stream = future
        .and_then(|w| {
            w.map_err(Error::from).and_then(|response| {
                let body = response.into_body().map_err(Error::from);

                Ok(body)
            })
        })
        .flatten_stream();

    let file = File::create(Path::new(filepath));

    file.map_err(Error::from).and_then(|file| {
        let write = FramedWrite::new(file, BytesCodec::new())
            .with(|chunk: Chunk| future::ok::<_, Error>(Bytes::from(chunk)));

        stream
            .forward(write)
            .and_then(|_| Ok(()))
            .map_err(Error::from)
    })
}

fn parse_single_field<T>(chunk: &Chunk, status: StatusCode) -> Result<T>
where
    T: for<'a> ::serde::Deserialize<'a> + Send + 'static,
{
    let body = str::from_utf8(chunk).map_err(Error::from)?;
    if status.is_success() {
        de_from_str::<T>(body).map_err(|_| ErrorKind::DockerApiUnknown(body.to_string(), status))
    } else {
        Err(match de_from_str::<ErrorResponse>(body) {
            Ok(x) => ErrorKind::DockerApi(x, status),
            Err(_) => ErrorKind::DockerApiUnknown(body.to_string(), status),
        })
    }
    .map_err(Error::from)
}

pub fn transform_stream(
    response: Response<Body>,
) -> impl Stream<Item = Bytes, Error = Error> + Send + 'static {
    let status = response.status();
    let body = response.into_body().map_err(Error::from);
    if status.is_success() {
        Box::new(body.map(|a| a.into_bytes()))
            as Box<Stream<Item = Bytes, Error = Error> + Send + 'static>
    } else {
        Box::new(
            body.concat2()
                .and_then(move |chunk: Chunk| {
                    str::from_utf8(&chunk)
                        .map_err(Error::from)
                        .and_then(|body| match de_from_str::<ErrorResponse>(body) {
                            Ok(x) => Err(ErrorKind::DockerApi(x, status).into()),
                            Err(_) => {
                                Err(ErrorKind::DockerApiUnknown(body.to_string(), status).into())
                            }
                        })
                })
                .into_stream(),
        )
    }
}

pub(crate) fn compose_uri(host: &Uri, path: &str, query: &str) -> Result<Uri> {
    let mut parts = host.clone().into_parts();
    let path_query = PathAndQuery::from_str(format!("{}?{}", path, query).as_ref())?;

    parts.path_and_query = Some(path_query);
    let res = Uri::from_parts(parts);

    debug!("URI parse result: {:?}", res);

    Ok(Uri::from(res?))
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
