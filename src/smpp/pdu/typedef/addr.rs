//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

/// Defines the Type of Number (TON) to be used in the SME address parameters.

/// The Type of Number (TON) to be used in the SME address parameters.
#[derive(Clone, Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u8)]
pub enum Ton {
    Unknown          = 0b00000000,
    International    = 0b00000001,
    National         = 0b00000010,
    NetworkSpecific  = 0b00000011,
    SubscriberNumber = 0b00000100,
    Alphanumeric     = 0b00000101,
    Abbreviated      = 0b00000110,
}

impl TryFrom<u8> for Ton {
    type Error = u8;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        use num_traits::FromPrimitive;

        FromPrimitive::from_u8(u).ok_or(u)
    }
}

impl From<Ton> for u8 {
    fn from(x: Ton) -> Self {
        use num_traits::ToPrimitive;

        x.to_u8().unwrap()
    }
}

impl bincode::Decode for Ton {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u8::decode(decoder)?;

        u.try_into()
            .map_err(|u: u8| bincode::error::DecodeError::UnexpectedVariant {
                type_name: "Ton",
                allowed: bincode::error::AllowedEnumVariants::Allowed(&[
                    0b00000000, 0b00000001, 0b00000010, 0b00000011, 0b00000100, 0b00000101, 0b00000110,
                ]),
                found: u as u32,
            })
    }
}

impl bincode::Encode for Ton {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u: u8 = self.clone().into();

        u.encode(encoder)
    }
}

/// The Numeric Plan Indicator (NPI) to be used in the SME address
#[derive(Clone, Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u8)]
pub enum Npi {
    Unknown     = 0b00000000,
    /// ISDN (E163/E164)
    ISDN        = 0b00000001,
    /// Data (X.121)
    Data        = 0b00000011,
    /// Telex (F.69)
    Telex       = 0b00000100,
    /// Land Mobile (E.212)
    LandMobile  = 0b00000110,
    National    = 0b00001000,
    Private     = 0b00001001,
    ERMES       = 0b00001010,
    /// Internet (IP)
    IP          = 0b00001110,
    /// WAP Client Id (to be defined by WAP Forum)
    WapClientId = 0b00010010,
}

impl TryFrom<u8> for Npi {
    type Error = u8;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        use num_traits::FromPrimitive;

        FromPrimitive::from_u8(u).ok_or(u)
    }
}

impl From<Npi> for u8 {
    fn from(x: Npi) -> Self {
        use num_traits::ToPrimitive;

        x.to_u8().unwrap()
    }
}

impl bincode::Decode for Npi {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u8::decode(decoder)?;

        u.try_into()
            .map_err(|u: u8| bincode::error::DecodeError::UnexpectedVariant {
                type_name: "Npi",
                allowed: bincode::error::AllowedEnumVariants::Allowed(&[
                    0b00000000, 0b00000001, 0b00000011, 0b00000100, 0b00000110, 0b00001000, 0b00001001, 0b00001010,
                    0b00001110, 0b00010010,
                ]),
                found: u as u32,
            })
    }
}

impl bincode::Encode for Npi {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u: u8 = self.clone().into();

        u.encode(encoder)
    }
}
