extern crate byteorder;

use self::byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use futures::Stream;
use Error;
use hyper::Chunk;
use futures::Async;
use futures::Poll;
use errors::ErrorKind;
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum State {
    Header,
    Body {
        message_size: usize,
        message_type: u32,
    }
}

struct TtyDecoder<T> where T : Stream<Item=Chunk> {
    state : State,
    buf : VecDeque<u8>,
    inner : T
}

impl<T> TtyDecoder<T> where T : Stream<Item=Chunk> {

    fn eat_buffer(&mut self) -> Poll<Option<(u32,Chunk)>, Error> {
        if let State::Header = self.state {
            if self.buf.len()>=8 {
                {
                    let header: Vec<u8> = self.buf.drain(0..8).collect();
                    debug!("{:?}", header);
                    let size = Cursor::new(&header[4..8])
                        .read_u32::<BigEndian>()? as usize;
                    self.state = State::Body {
                        message_size: size,
                        message_type: header[0] as u32,
                    }
                }
            }
            else {
                return Ok(Async::NotReady)
            }
        }

        if let State::Body { message_size, message_type } = self.state {
            if self.buf.len() >= message_size {
                let chunk : Vec<u8> = self.buf.drain(0..message_size).collect();
                self.state = State::Header;
                return Ok(Async::Ready(Some((message_type, chunk.into()))))
            }
            else {
                return Ok(Async::NotReady)
            }
        }

        unreachable!()
    }

}

impl<T> Stream for TtyDecoder<T> where T : Stream<Item=Chunk>, Error : From<T::Error> {
    type Item = (u32, Chunk);
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.eat_buffer() {
                v @ Ok(Async::Ready(Some(_))) => return v,
                v @ Err(_) => return v,
                _ => ()
            };

            let it: Option<Chunk> = match self.inner.poll() {
                Ok(Async::NotReady) => {
                    debug!("not_ready");
                    return Ok(Async::NotReady)
                },
                Ok(Async::Ready(item)) => item,
                Err(e) => return Err(e.into())
            };

            if let Some(chunk) = it {
                for b in chunk {
                    self.buf.push_back(b);
                }
            }
            else {
                if self.buf.len() == 0 {
                    return Ok(Async::Ready(None))
                }
                else {
                    return Err(ErrorKind::Eof.into())
                }
            }
        }
    }
}

pub fn decode<F>(stream : F) -> impl Stream<Item=(u32, Chunk), Error=Error> where F : Stream<Item=Chunk, Error=Error> {
    TtyDecoder { state: State::Header, buf: VecDeque::new(), inner: stream }
}