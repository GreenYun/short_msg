// Copyright (c) 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

pub use gsm::{decode as gsm_decode, encode as gsm_encode};

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum DataCoding {
    /// SMSC Default Alphabet (GSM 03.38 default 7-bit)
    #[cfg(not(feature = "v5"))]
    SmscDefault, /* = 0b00000000 */
    /// MC Specific
    #[cfg(feature = "v5")]
    McSpecific, /* = 0b00000000 */
    /// ASCII
    Ascii, /* = 0b00000001 */
    /// ISO-8859-1 (Latin-1)
    Latin1, /* = 0b00000011 */
    /// JIS (X 0208-1990)
    ShiftJis, /* = 0b00000101 */
    /// ISO-8859-5 (Cyrillic)
    Cyrillic, /* = 0b00000110 */
    /// ISO-8859-8 (Hebrew)
    Hebrew, /* = 0b00000111 */
    /// ISO/IEC-10646 (UCS2)
    Ucs2, /* = 0b00001000 */
    /// ISO-2022-JP (Music Codes)
    Iso2022Jp, /* = 0b00001010 */
    /// Extended Kanji JIS (X 0212-1990)
    EucJp, /* = 0b00001101 */
    /// KS X 1001 (KS C 5601)
    EucKr, /* = 0b00001110 */

    Reserved(u8),
}

impl bincode::Decode for DataCoding {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u8::decode(decoder)?;

        Ok(match u {
            #[cfg(not(feature = "v5"))]
            0b00000000 => Self::SmscDefault,
            #[cfg(feature = "v5")]
            0b00000000 => Self::McSpecific,
            0b00000001 => Self::Ascii,
            0b00000011 => Self::Latin1,
            0b00000101 => Self::ShiftJis,
            0b00000110 => Self::Cyrillic,
            0b00000111 => Self::Hebrew,
            0b00001000 => Self::Ucs2,
            0b00001010 => Self::Iso2022Jp,
            0b00001101 => Self::EucJp,
            0b00001110 => Self::EucKr,
            x => Self::Reserved(x),
        })
    }
}

impl bincode::Encode for DataCoding {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u = match self {
            #[cfg(not(feature = "v5"))]
            Self::SmscDefault => 0b00000000,
            #[cfg(feature = "v5")]
            Self::McSpecific => 0b00000000,
            Self::Ascii => 0b00000001,
            Self::Latin1 => 0b00000011,
            Self::ShiftJis => 0b00000101,
            Self::Cyrillic => 0b00000110,
            Self::Hebrew => 0b00000111,
            Self::Ucs2 => 0b00001000,
            Self::Iso2022Jp => 0b00001010,
            Self::EucJp => 0b00001101,
            Self::EucKr => 0b00001110,
            Self::Reserved(x) => *x,
        };

        u.encode(encoder)
    }
}

mod gsm;
