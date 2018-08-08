extern crate byteorder;

use self::byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::io::Read;
use futures::Stream;
use Error;
use tokio_codec::FramedWrite;
use std::io::Sink;
use tokio_codec::Decoder;
use bytes::BytesMut;
use tokio::io;
use std::str;
use hyper::client::ResponseFuture;
use tokio_codec::FramedRead;
use transport::parse::ResponseFutureWrapper;
use futures::Future;
use tokio::prelude::future;
use hyper::Chunk;
use tokio_codec::Framed;
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

struct PacketDecoder {
    next_index: usize,
    state: State,
}

impl PacketDecoder {
    pub fn new() -> PacketDecoder {
        PacketDecoder { next_index: 0, state: State::Header, }
    }
}

fn utf8(buf: &[u8]) -> Result<&str, io::Error> {
    str::from_utf8(buf).map_err(|_|
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Unable to decode input as UTF8"))
}

impl Decoder for PacketDecoder {
    type Item = String;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<String>, Error> {
        let current_size = buf.len() - self.next_index;
        if self.state == State::Header && current_size >= 8 {
            let header = &buf[self.next_index..self.next_index+8];
            let size = Cursor::new(&header[4..8])
                .read_u32::<BigEndian>()
                .unwrap() as usize;

            self.next_index += 8;
            self.state = State::Body {
                message_size: size,
                message_type: header[0] as u32,
            };
        }

        let current_size = buf.len() - self.next_index;
        if let State::Body {message_size, message_type} = self.state {
            if current_size >= message_size {
                let index = message_size + self.next_index;
                let line = buf.split_to(index + 1);
                let line : &[u8] = &line[..index];
                let line = utf8(line)?;

                self.next_index = 0;
                self.state = State::Header;

                return Ok(Some(line.to_string()))
            }
        }

        Ok(None)
    }
}

struct TtyDecoder<T> where T : Stream<Item=Chunk> {
    state : State,
    buf : VecDeque<u8>,
    inner : T
}

impl<T> TtyDecoder<T> where T : Stream<Item=Chunk> {

    fn eat_buffer(&mut self) -> Poll<Option<(u32,Chunk)>, Error> {

        debug!(">>> Eat");
        debug!("<< {:?}", self.state);
        if let State::Header = self.state {
            debug!(">>> HEADER");
            debug!("Buff: {:?}", self.buf);
            if self.buf.len()>=8 {
                {
                    let header: Vec<u8> = self.buf.drain(0..8).collect();
                    debug!("{:?}", header);
                    let size = Cursor::new(&header[4..8])
                        .read_u32::<BigEndian>()
                        .unwrap() as usize;
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
            debug!(">>> BODY");
            debug!("Buff: {:?}", self.buf);
            if self.buf.len() >= message_size {
                let chunk : Vec<u8> = self.buf.drain(0..message_size).collect();
                self.state = State::Header;
                return Ok(Async::Ready(Some((message_type, chunk.into()))))
            }
            else {
                debug!(">>> Not ready");
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
        use bytes::*;

        debug!(">>> Poll");
        loop {
            match self.eat_buffer() {
                v @ Ok(Async::Ready(Some(_))) => return v,
                v @ Err(_) => return v,
                _ => ()
            };

            debug!(">>> Further");
            let it: Option<Chunk> = match self.inner.poll() {
                Ok(Async::NotReady) => {
                    debug!("not_ready");
                    return Ok(Async::NotReady)
                },
                Ok(Async::Ready(item)) => item,
                Err(e) => return Err(e.into())
            };

            debug!(">>> Ready");

            if let Some(chunk) = it {
                for b in chunk {
                    self.buf.push_back(b);
                }
            }
            else {
                debug!(">>> {}", self.buf.len());
                debug!(">>> {:?}", self.state);
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

pub fn decode<F>(mut stream : F) -> impl Stream<Item=(u32, Chunk), Error=Error> where F : Stream<Item=Chunk, Error=Error> {
    debug!(">>> Decode");

    TtyDecoder { state: State::Header, buf: VecDeque::new(), inner: stream }
}