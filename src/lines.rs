use futures;

use std::mem::replace;
use std::string::FromUtf8Error;

use futures::{Async, Poll, Stream, Future};
use futures::stream::{Fuse};

use std::iter::Iterator;

pub(crate) struct Lines<S: Stream> {
    buffered: Option<Vec<u8>>,
    stream: Fuse<S>
}

impl<S: Stream> Lines<S> {
    pub fn new(stream: S) -> Lines<S> {
        Lines {
            buffered: None,
            stream: stream.fuse()
        }
    }

    fn process(&mut self, flush: bool) -> Option<Result<String, FromUtf8Error>> {
        let buffered = replace(&mut self.buffered, None);
        if let Some(ref buffer) = buffered {
            let mut split = buffer.splitn(2, |c| *c == b'\n');
            if let Some(first) = split.next() {
                if let Some(second) = split.next() {
                    replace(&mut self.buffered, Some(second.to_vec()));
                    return Some(String::from_utf8(first.to_vec()));
                } else if flush {
                    return Some(String::from_utf8(first.to_vec()));
                }
            }
        }
        replace(&mut self.buffered, buffered);
        None
    }
}

impl<S> Stream for Lines<S>
    where S: Stream, S::Item: AsRef<[u8]>, S::Error: From<FromUtf8Error>
{
    type Item = String;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<String>, S::Error> {
        match self.stream.poll()? {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(None) => {
                match self.process(true) {
                    Some(Ok(line)) => Ok(Async::Ready(Some(line))),
                    Some(Err(err)) => Err(err.into()),
                    None => Ok(Async::Ready(None))
                }
            },
            Async::Ready(Some(chunk)) => {
                if let Some(ref mut buffer) = self.buffered {
                    buffer.extend(chunk.as_ref());
                } else {
                    self.buffered = Some(chunk.as_ref().to_vec());
                }
                match self.process(false) {
                    Some(Ok(line)) => Ok(Async::Ready(Some(line))),
                    Some(Err(err)) => Err(err.into()),
                    None => Ok(Async::NotReady)
                }
            }
        }
    }
}
