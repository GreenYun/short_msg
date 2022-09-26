// Copyright (c) 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::convert::TryInto;

pub use gsm::{decode as gsm_decode, encode as gsm_encode};

#[derive(Clone, Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u8)]
pub enum DataCoding {
    /// SMSC Default Alphabet (GSM 03.38 default 7-bit)
    #[cfg(not(feature = "v5"))]
    SmscDefault = 0b00000000,
    /// MC Specific
    #[cfg(feature = "v5")]
    McSpecific  = 0b00000000,
    /// ASCII
    Ascii       = 0b00000001,
    /// ISO-8859-1 (Latin-1)
    Latin1      = 0b00000011,
    /// JIS (X 0208-1990)
    ShiftJis    = 0b00000101,
    /// ISO-8859-5 (Cyrillic)
    Cyrillic    = 0b00000110,
    /// ISO-8859-8 (Hebrew)
    Hebrew      = 0b00000111,
    /// ISO/IEC-10646 (UCS2)
    Ucs2        = 0b00001000,
    /// ISO-2022-JP (Music Codes)
    Iso2022Jp   = 0b00001010,
    /// Extended Kanji JIS (X 0212-1990)
    EucJp       = 0b00001101,
    /// KS X 1001 (KS C 5601)
    EucKr       = 0b00001110,
}

impl TryFrom<u8> for DataCoding {
    type Error = u8;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        use num_traits::FromPrimitive;

        FromPrimitive::from_u8(u).ok_or(u)
    }
}

impl From<DataCoding> for u8 {
    fn from(x: DataCoding) -> Self {
        use num_traits::ToPrimitive;

        x.to_u8().unwrap_or_default()
    }
}

impl bincode::Decode for DataCoding {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u8::decode(decoder)?;

        u.try_into()
            .map_err(|u: u8| bincode::error::DecodeError::UnexpectedVariant {
                type_name: "DataCoding",
                allowed: bincode::error::AllowedEnumVariants::Allowed(&[]),
                found: u32::from(u),
            })
    }
}

impl bincode::Encode for DataCoding {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u: u8 = self.clone().into();

        u.encode(encoder)
    }
}

mod gsm;
