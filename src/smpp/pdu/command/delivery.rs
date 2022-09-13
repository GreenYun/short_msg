//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

//! Message delivery operations provide the means of delivering short messages
//! from a MC to an ESME. These messages typically originate from mobile
//! stations.

use crate::smpp::pdu::typedef::{COctet, TLV};

/// The deliver_sm is issued by the SMSC (v5: MC) to send a message to an ESME.
/// Using this command, the SMSC (v5: MC) may route a short message to the ESME
/// for delivery.
#[derive(Clone, Debug)]
pub struct DeliverSm {
    pub service_type: COctet,
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: COctet,
    pub dest_addr_ton: u8,
    pub dest_addr_npi: u8,
    pub destination_addr: COctet,
    pub esm_class: u8,
    pub protocol_id: u8,
    pub priority_flag: u8,
    pub schedule_delivery_time: COctet,
    pub validity_period: COctet,
    pub registered_delivery: u8,
    pub replace_if_present_flag: u8,
    pub data_coding: u8,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub msg_delivery_tlv: Vec<TLV>,
}

impl bincode::Decode for DeliverSm {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let service_type = COctet::decode(decoder)?;
        let source_addr_ton = u8::decode(decoder)?;
        let source_addr_npi = u8::decode(decoder)?;
        let source_addr = COctet::decode(decoder)?;
        let dest_addr_ton = u8::decode(decoder)?;
        let dest_addr_npi = u8::decode(decoder)?;
        let destination_addr = COctet::decode(decoder)?;
        let esm_class = u8::decode(decoder)?;
        let protocol_id = u8::decode(decoder)?;
        let priority_flag = u8::decode(decoder)?;
        let schedule_delivery_time = COctet::decode(decoder)?;
        let validity_period = COctet::decode(decoder)?;
        let registered_delivery = u8::decode(decoder)?;
        let replace_if_present_flag = u8::decode(decoder)?;
        let data_coding = u8::decode(decoder)?;
        let sm_default_msg_id = u8::decode(decoder)?;
        let sm_length = u8::decode(decoder)?;

        let mut short_message = vec![];
        for _ in 0..sm_length {
            let u = u8::decode(decoder)?;
            short_message.push(u);
        }

        let msg_delivery_tlv = {
            let mut v = vec![];
            while let Ok(t) = TLV::decode(decoder) {
                v.push(t);
            }
            v
        };

        Ok(Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            protocol_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            sm_length,
            short_message,
            msg_delivery_tlv,
        })
    }
}

impl bincode::Encode for DeliverSm {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        use bincode::enc::write::Writer;

        self.service_type.encode(encoder)?;
        self.source_addr_ton.encode(encoder)?;
        self.source_addr_npi.encode(encoder)?;
        self.source_addr.encode(encoder)?;
        self.dest_addr_ton.encode(encoder)?;
        self.dest_addr_npi.encode(encoder)?;
        self.destination_addr.encode(encoder)?;
        self.esm_class.encode(encoder)?;
        self.protocol_id.encode(encoder)?;
        self.priority_flag.encode(encoder)?;
        self.schedule_delivery_time.encode(encoder)?;
        self.validity_period.encode(encoder)?;
        self.registered_delivery.encode(encoder)?;
        self.replace_if_present_flag.encode(encoder)?;
        self.data_coding.encode(encoder)?;
        self.sm_default_msg_id.encode(encoder)?;
        self.sm_length.encode(encoder)?;
        encoder.writer().write(&self.short_message)?;

        for t in &self.msg_delivery_tlv {
            t.encode(encoder)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct DeliverSmResp {
    message_id: COctet,

    pub msg_delivery_resp_tlv: Vec<TLV>,
}

impl bincode::Decode for DeliverSmResp {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let message_id = COctet::decode(decoder)?;

        let msg_delivery_resp_tlv = {
            let mut v = vec![];
            while let Ok(t) = TLV::decode(decoder) {
                v.push(t);
            }
            v
        };

        Ok(Self {
            message_id,
            msg_delivery_resp_tlv,
        })
    }
}

impl bincode::Encode for DeliverSmResp {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        self.message_id.encode(encoder)?;

        #[cfg(feature = "v5")]
        for t in &self.msg_delivery_resp_tlv {
            t.encode(encoder)?;
        }

        Ok(())
    }
}

/// The deliver_sm is issued by the SMSC (v5: MC) to send a message to an ESME.
/// Using this command, the SMSC (v5: MC) may route a short message to the ESME
/// for delivery.
#[derive(Clone, Debug)]
pub struct DataSm {
    pub service_type: COctet,
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: COctet,
    pub dest_addr_ton: u8,
    pub dest_addr_npi: u8,
    pub destination_addr: COctet,
    pub esm_class: u8,
    pub registered_delivery: u8,
    pub data_coding: u8,
    pub msg_submission_tlv: Vec<TLV>,
}

impl bincode::Decode for DataSm {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let service_type = COctet::decode(decoder)?;
        let source_addr_ton = u8::decode(decoder)?;
        let source_addr_npi = u8::decode(decoder)?;
        let source_addr = COctet::decode(decoder)?;
        let dest_addr_ton = u8::decode(decoder)?;
        let dest_addr_npi = u8::decode(decoder)?;
        let destination_addr = COctet::decode(decoder)?;
        let esm_class = u8::decode(decoder)?;
        let registered_delivery = u8::decode(decoder)?;
        let data_coding = u8::decode(decoder)?;

        let msg_submission_tlv = {
            let mut v = vec![];
            while let Ok(t) = TLV::decode(decoder) {
                v.push(t);
            }
            v
        };

        Ok(Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            registered_delivery,
            data_coding,
            msg_submission_tlv,
        })
    }
}

impl bincode::Encode for DataSm {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        self.service_type.encode(encoder)?;
        self.source_addr_ton.encode(encoder)?;
        self.source_addr_npi.encode(encoder)?;
        self.source_addr.encode(encoder)?;
        self.dest_addr_ton.encode(encoder)?;
        self.dest_addr_npi.encode(encoder)?;
        self.destination_addr.encode(encoder)?;
        self.esm_class.encode(encoder)?;
        self.registered_delivery.encode(encoder)?;
        self.data_coding.encode(encoder)?;

        for t in &self.msg_submission_tlv {
            t.encode(encoder)?;
        }

        Ok(())
    }
}

pub type DataSmResp = DeliverSmResp;
