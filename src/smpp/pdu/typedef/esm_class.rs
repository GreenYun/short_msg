//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

use std::convert::TryInto;

/// Used to indicate special message attributes associated with the short
/// message.
#[derive(Clone, Debug)]
pub struct EsmClass {
    pub message_mode: EsmClassMessageMode,
    pub message_type: EsmClassMessageType,
    pub ansi41: EsmClassAnsi41,
    pub gsm: EsmClassGsm,
}

impl TryFrom<u8> for EsmClass {
    type Error = u8;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        use num_traits::FromPrimitive;

        let message_mode = u & 0b00000011;
        let message_mode = FromPrimitive::from_u8(message_mode).ok_or(u)?;

        let message_type = u & 0b00100100;
        let message_type = FromPrimitive::from_u8(message_type).ok_or(u)?;

        let ansi41 = u & 0b00011000;
        let ansi41 = FromPrimitive::from_u8(ansi41).ok_or(u)?;

        let gsm = u & 0b11000000;
        let gsm = FromPrimitive::from_u8(gsm).ok_or(u)?;

        Ok(Self {
            message_mode,
            message_type,
            ansi41,
            gsm,
        })
    }
}

impl From<EsmClass> for u8 {
    fn from(x: EsmClass) -> Self {
        use num_traits::ToPrimitive;

        let message_mode = x.message_mode.to_u8().unwrap();
        let message_type = x.message_type.to_u8().unwrap();
        let ansi41 = x.ansi41.to_u8().unwrap();
        let gsm = x.gsm.to_u8().unwrap();

        message_mode + message_type + ansi41 + gsm
    }
}

impl bincode::Decode for EsmClass {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u8::decode(decoder)?;

        u.try_into()
            .map_err(|u: u8| bincode::error::DecodeError::UnexpectedVariant {
                type_name: "EsmClass",
                allowed: bincode::error::AllowedEnumVariants::Allowed(&[]),
                found: u as u32,
            })
    }
}

impl bincode::Encode for EsmClass {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u: u8 = self.clone().into();
        u.encode(encoder)
    }
}

#[derive(Clone, Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u8)]
pub enum EsmClassMessageMode {
    /// Default MC Mode (e.g. Store and Forward)
    Default      = 0b00000000,
    /// Datagram mode
    Datagram     = 0b00000001,
    /// Forward (i.e. Transaction) mode
    Forward      = 0b00000010,
    /// Store and Forward mode (use to select Store and Forward mode if Default
    /// MC Mode is non Store and Forward)
    StoreForward = 0b00000011,
}

#[derive(Clone, Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u8)]
pub enum EsmClassMessageType {
    /// Default message Type (i.e. normal message)
    Default                          = 0b00000000,
    /// Short Message contains MC Delivery Receipt
    DeliveryReceipt                  = 0b00000100,
    /// Short Message contains Intermediate Delivery Notification
    IntermediateDeliveryNotification = 0b00100000,
}

#[derive(Clone, Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u8)]
pub enum EsmClassAnsi41 {
    None        = 0b00000000,
    /// Short Message contains Delivery Acknowledgement
    DeliveryAck = 0b00001000,
    /// Short Message contains Manual/User Acknowledgement
    ManualAck   = 0b00010000,
    /// Short Message contains Conversation Abort (Korean CDMA)
    Abort       = 0b00011000,
}

#[derive(Clone, Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u8)]
pub enum EsmClassGsm {
    /// No specific features selected
    None             = 0b00000000,
    /// UDH Indicator
    UDHI             = 0b01000000,
    /// Set Reply Path (only relevant for GSM network)
    ReplyPath        = 0b10000000,
    /// Set UDHI and Reply Path (only relevant for GSM network)
    UDHIandReplyPath = 0b11000000,
}
