/*
 * Copyright 2017-2018 Ben Ashford
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::io;
use std::net::SocketAddr;

use futures::Future;

use tokio_codec::{Decoder, Framed};

use tokio_tcp::TcpStream;

use crate::resp;

pub type RespConnection = Framed<TcpStream, resp::RespCodec>;

/// Connect to a Redis server and return a Future that resolves to a
/// `RespConnection` for reading and writing asynchronously.
///
/// Each `RespConnection` implements both `Sink` and `Stream` and read and
/// writes `RESP` objects.
///
/// This is a low-level interface to enable the creation of higher-level
/// functionality.
///
/// The sink and stream sides behave independently of each other, it is the
/// responsibility of the calling application to determine what results are
/// paired to a particular command.
///
/// But since most Redis usages involve issue commands that result in one
/// single result, this library also implements `paired_connect`.
pub fn connect(addr: &SocketAddr) -> impl Future<Item = RespConnection, Error = io::Error> {
    TcpStream::connect(addr).map(move |socket| resp::RespCodec.framed(socket))
}
