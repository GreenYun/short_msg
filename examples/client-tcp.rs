//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

extern crate bincode;
extern crate bytes;
extern crate short_msg;
extern crate tokio;

use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use short_msg::smpp::pdu::{prelude::*, Header};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();

    let mut stream = TcpStream::connect("127.0.0.1:34254").await?;

    let header = Header::new(16, Id::EnquireLink, Status::ESME_ROK, 1);
    // let enquire_link = EnquireLink {};
    let enquire_link = bincode::encode_to_vec(header, config)?;

    stream.write_all(&enquire_link).await?;

    let data_len = stream.read_u32().await?;

    let mut buf = BytesMut::with_capacity(data_len as usize);
    stream.read_buf(&mut buf).await?;

    let mut resp = buf.to_vec();
    resp.splice(0..0, bincode::encode_to_vec(data_len, config)?);
    let (resp, _) = bincode::decode_from_slice::<Header, _>(&resp, config)?;

    println!("{:?}", resp);

    Ok(())
}
