//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

use std::ffi::{CStr, CString, NulError};

#[derive(Clone, Debug, Default)]
pub struct COctet {
    inner: CString,
}

impl COctet {
    pub fn new<T>(t: T) -> Result<Self, NulError>
    where
        T: Into<Vec<u8>>,
    {
        Ok(Self {
            inner: CString::new(t)?,
        })
    }

    pub fn from_c_string(s: CString) -> Self {
        Self { inner: s }
    }

    pub fn as_c_string(self) -> CString {
        self.inner
    }
}

impl AsRef<CStr> for COctet {
    fn as_ref(&self) -> &CStr {
        &self.inner
    }
}

impl bincode::Decode for COctet {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let mut t = Vec::with_capacity(128);
        loop {
            let b = u8::decode(decoder)?;
            if b == 0x00 {
                break;
            }

            t.push(b);
        }

        // We have ensured that no NulError will be returned from `CString::new`.
        Ok(Self::new(t).unwrap())
    }
}

impl bincode::Encode for COctet {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        use bincode::enc::write::Writer;

        encoder.writer().write(self.inner.as_bytes_with_nul())
    }
}
