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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum State {
    Header,
    Body {
        message_size: usize,
        message_type: u8,
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
                message_type: header[0],
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
/*
// https://docs.docker.com/engine/api/v1.26/#operation/ContainerAttach
pub fn tty(stream: ResponseFutureWrapper)
        -> Box<Stream<Item=String, Error=Error> + Send>
{
    let stream = stream
        .and_then(|f| f.map_err(Error::from))
        .and_then(|response| future::ok(response.into_body().forward()).map_err(Error::from))
        .map_err(Error::from)
        .flatten_stream().framed();

    let reader = FramedRead::new(stream, PacketDecoder::new());

    Box::new(reader)
}
*/