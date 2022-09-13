//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

extern crate bincode;
extern crate bytes;
extern crate short_msg;
extern crate tokio;

use std::env::args;

use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use short_msg::smpp::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = args().nth(1).expect("At least one argument should be specified.");

    let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();

    let mut stream = TcpStream::connect(server).await?;

    let header = Header::new(Id::EnquireLink, Status::ESME_ROK, 1);

    {
        // let enquire_link = EnquireLink {};
        let enquire_link = bincode::encode_to_vec(header.clone(), config)?;

        stream.write_all(&enquire_link).await?;

        let data_len = stream.read_u32().await?;

        let mut buf = BytesMut::with_capacity(data_len as usize);
        stream.read_buf(&mut buf).await?;

        let mut resp = buf.to_vec();
        resp.splice(0..0, bincode::encode_to_vec(data_len, config)?);
        let (resp, _) = bincode::decode_from_slice::<Header, _>(&resp, config)?;

        println!("{:?}", resp);
    }

    {
        let bind_transceiver = BindTransceiver {
            system_id: COctet::new("Telegram")?,
            password: COctet::new("")?,
            system_type: COctet::new("")?,
            interface_version: 0x34,
            addr_ton: 0,
            addr_npi: 0,
            address_range: COctet::new("")?,
        };

        let bind_transceiver = Header::new_with_body(Id::BindTransceiver, Status::ESME_ROK, 2, bind_transceiver);

        stream.write_all(&bind_transceiver).await?;

        let data_len = stream.read_u32().await?;

        let mut buf = BytesMut::with_capacity(data_len as usize);
        stream.read_buf(&mut buf).await?;

        let mut resp = buf.to_vec();
        resp.splice(0..0, bincode::encode_to_vec(data_len, config)?);
        let (header_resp, _) = bincode::decode_from_slice::<Header, _>(&resp, config)?;
        println!("{:?}", header_resp);

        if matches!(header_resp.command_id, Id::BindTransceiverResp)
            && matches!(header_resp.command_status, Status::ESME_ROK)
        {
            let (resp, _) = bincode::decode_from_slice::<BindTransceiverResp, _>(&resp[16..], config)?;
            println!("{:?}", resp);

            let header = Header::new(Id::Unbind, Status::ESME_ROK, 3);
            let unbind = bincode::encode_to_vec(header, config)?;

            stream.write_all(&unbind).await?;

            let data_len = stream.read_u32().await?;
            let mut buf = BytesMut::with_capacity(data_len as usize);
            stream.read_buf(&mut buf).await?;

            let mut resp = buf.to_vec();
            resp.splice(0..0, bincode::encode_to_vec(data_len, config)?);
            let (resp, _) = bincode::decode_from_slice::<Header, _>(&resp, config)?;
            println!("{:?}", resp);
        }
    }

    Ok(())
}
