// Copyright (c) 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

#[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
pub struct Header {
    pub command_length: u32,
    pub command_id: command::Id,
    pub command_status: command::Status,
    pub sequence_number: u32,
}

impl Header {
    #[must_use]
    pub const fn new(id: command::Id, status: command::Status, seq_num: u32) -> Self {
        Self {
            command_length: 16,
            command_id: id,
            command_status: status,
            sequence_number: seq_num,
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn new_with_body<E>(
        id: command::Id,
        status: command::Status,
        seq_num: u32,
        body: E,
    ) -> Result<Vec<u8>, bincode::error::EncodeError>
    where
        E: bincode::Encode,
    {
        let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();
        let v = bincode::encode_to_vec(body, config)?;

        #[allow(clippy::cast_possible_truncation)]
        let header = Self::new(id, status, seq_num).set_len(16 + v.len() as u32);
        let mut res = bincode::encode_to_vec(header, config)?;
        res.extend_from_slice(&v);
        Ok(res)
    }

    const fn set_len(self, len: u32) -> Self {
        Self {
            command_length: len,
            command_id: self.command_id,
            command_status: self.command_status,
            sequence_number: self.sequence_number,
        }
    }
}

pub mod command;
pub mod typedef;
