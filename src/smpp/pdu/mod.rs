//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

#[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
pub struct Header {
    pub command_length: u32,
    pub command_id: command::Id,
    pub command_status: command::Status,
    pub sequence_number: u32,
}

impl Header {
    pub fn new(id: command::Id, status: command::Status, seq_num: u32) -> Self {
        Self {
            command_length: 16,
            command_id: id,
            command_status: status,
            sequence_number: seq_num,
        }
    }

    pub fn new_with_body<E>(id: command::Id, status: command::Status, seq_num: u32, body: E) -> Vec<u8>
    where
        E: bincode::Encode,
    {
        let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();
        let v = bincode::encode_to_vec(body, config).unwrap();
        let header = Header::new(id, status, seq_num).set_len(16 + v.len() as u32);
        let mut res = bincode::encode_to_vec(header, config).unwrap();
        res.extend_from_slice(&v);
        res
    }

    fn set_len(self, len: u32) -> Self {
        Self {
            command_length: len,
            command_id: self.command_id,
            command_status: self.command_status,
            sequence_number: self.sequence_number,
        }
    }
}

pub mod prelude {
    pub use super::{command::*, data_coding::*, typedef::*};
}

pub mod command;
pub mod data_coding;
pub mod typedef;
