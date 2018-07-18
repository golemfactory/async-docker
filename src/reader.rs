//! Helper iterator for streaming requests
//!
//! Source of code without generic types:
//! https://github.com/faradayio/boondock/blob/master/src/stats.rs

use errors::*;
use serde::de::DeserializeOwned;
use serde_json;
use std::marker::PhantomData;
use std::io::BufReader;
use hyper::Response;
use std::iter;
use hyper::Body;
/*

pub struct BufIterator<T: DeserializeOwned> {
    buf: BufReader<Body>,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> BufIterator<T> {
    pub fn new(r: Response<Body>) -> BufIterator<T> {
        BufIterator {
            buf: BufReader::new(r.into_body()),
            _phantom: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> iter::Iterator for BufIterator<T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Result<T>> {
        let mut line = String::new();

        match self.buf.read_line(&mut line) {
            // Error while reading
            Err(err) => Some(Err(err.into())),
            // found EOF
            Ok(0) => None,
            Ok(_) => Some(serde_json::from_str::<T>(&line).map_err(Error::from)),
        }
    }
}
*/