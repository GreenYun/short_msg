//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

//! Message submission operations provide an ESME with the ability to submit
//! messages for onward delivery to mobile stations.

use crate::smpp::pdu::typedef::{COctet, TLV};

/// This operation is used by an ESME to submit a short message to the SMSC (v5:
/// MC) for onward transmission to a specified short message entity (SME).
#[derive(Clone, Debug)]
pub struct SubmitSm {
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
    pub msg_submission_tlv: Vec<TLV>,
}

impl bincode::Decode for SubmitSm {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        use bincode::de::read::Reader;

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

        let mut short_message = vec![Default::default(); sm_length.into()];
        decoder.reader().read(&mut short_message)?;

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
            msg_submission_tlv,
        })
    }
}

impl bincode::Encode for SubmitSm {
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

        for t in &self.msg_submission_tlv {
            t.encode(encoder)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SubmitSmResp {
    message_id: COctet,

    pub msg_submission_resp_tlv: Vec<TLV>,
}

impl bincode::Decode for SubmitSmResp {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let message_id = COctet::decode(decoder)?;

        let msg_submission_resp_tlv = {
            let mut v = vec![];
            while let Ok(t) = TLV::decode(decoder) {
                v.push(t);
            }
            v
        };

        Ok(Self {
            message_id,
            msg_submission_resp_tlv,
        })
    }
}

impl bincode::Encode for SubmitSmResp {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        self.message_id.encode(encoder)?;

        for t in &self.msg_submission_resp_tlv {
            t.encode(encoder)?;
        }

        Ok(())
    }
}
