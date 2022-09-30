// Copyright (c) 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

/// Defines the Type of Number (TON) to be used in the SME address parameters.

/// The Type of Number (TON) to be used in the SME address parameters.
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Ton {
    Unknown,          /* = 0b00000000 */
    International,    /* = 0b00000001 */
    National,         /* = 0b00000010 */
    NetworkSpecific,  /* = 0b00000011 */
    SubscriberNumber, /* = 0b00000100 */
    Alphanumeric,     /* = 0b00000101 */
    Abbreviated,      /* = 0b00000110 */
    Reserved(u8),
}

impl bincode::Decode for Ton {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u8::decode(decoder)?;

        Ok(match u {
            0b00000000 => Self::Unknown,
            0b00000001 => Self::International,
            0b00000010 => Self::National,
            0b00000011 => Self::NetworkSpecific,
            0b00000100 => Self::SubscriberNumber,
            0b00000101 => Self::Alphanumeric,
            0b00000110 => Self::Abbreviated,
            x => Self::Reserved(x),
        })
    }
}

impl bincode::Encode for Ton {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u: u8 = match self {
            Self::Unknown => 0b00000000,
            Self::International => 0b00000001,
            Self::National => 0b00000010,
            Self::NetworkSpecific => 0b00000011,
            Self::SubscriberNumber => 0b00000100,
            Self::Alphanumeric => 0b00000101,
            Self::Abbreviated => 0b00000110,
            Self::Reserved(x) => *x,
        };

        u.encode(encoder)
    }
}

/// The Numeric Plan Indicator (NPI) to be used in the SME address
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Npi {
    Unknown, /* = 0b00000000 */
    /// ISDN (E163/E164)
    ISDN, /* = 0b00000001 */
    /// Data (X.121)
    Data, /* = 0b00000011 */
    /// Telex (F.69)
    Telex, /* = 0b00000100 */
    /// Land Mobile (E.212)
    LandMobile, /* = 0b00000110 */
    National, /* = 0b00001000 */
    Private, /* = 0b00001001 */
    ERMES,   /* = 0b00001010 */
    /// Internet (IP)
    IP, /* = 0b00001110 */
    /// WAP Client Id (to be defined by WAP Forum)
    WapClientId, /* = 0b00010010 */
    Reserved(u8),
}

impl bincode::Decode for Npi {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u8::decode(decoder)?;

        Ok(match u {
            0b00000000 => Self::Unknown,
            0b00000001 => Self::ISDN,
            0b00000011 => Self::Data,
            0b00000100 => Self::Telex,
            0b00000110 => Self::LandMobile,
            0b00001000 => Self::National,
            0b00001001 => Self::Private,
            0b00001010 => Self::ERMES,
            0b00001110 => Self::IP,
            0b00010010 => Self::WapClientId,
            x => Self::Reserved(x),
        })
    }
}

impl bincode::Encode for Npi {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u: u8 = match self {
            Self::Unknown => 0b00000000,
            Self::ISDN => 0b00000001,
            Self::Data => 0b00000011,
            Self::Telex => 0b00000100,
            Self::LandMobile => 0b00000110,
            Self::National => 0b00001000,
            Self::Private => 0b00001001,
            Self::ERMES => 0b00001010,
            Self::IP => 0b00001110,
            Self::WapClientId => 0b00010010,
            Self::Reserved(x) => *x,
        };

        u.encode(encoder)
    }
}
