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
    pub fn new(len: u32, id: command::Id, status: command::Status, seq_num: u32) -> Self {
        Self {
            command_length: len,
            command_id: id,
            command_status: status,
            sequence_number: seq_num,
        }
    }

    pub fn advance(self) -> Self {
        Self {
            command_length: self.command_length,
            command_id: self.command_id,
            command_status: self.command_status,
            sequence_number: self.sequence_number + 1,
        }
    }

    pub fn set_id(self, id: command::Id) -> Self {
        Self {
            command_length: self.command_length,
            command_id: id,
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
