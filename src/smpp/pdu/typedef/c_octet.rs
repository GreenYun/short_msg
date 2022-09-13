//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

use std::ffi::{CStr, CString, NulError};

/// A C-Octet String is a sequence of ASCII characters terminated with a NULL
/// octet (`0x00`).
///
/// # Note:
///
/// By default, [`CString`] is encoded by [`bincode`] as
/// [Collection](https://github.com/bincode-org/bincode/blob/trunk/docs/spec.md#Collections) type.
/// Thus we wrap that in a new type.
#[derive(Clone, Debug, Default)]
pub struct COctet {
    inner: CString,
}

impl COctet {
    /// Creates a new C-compatible string from a container of bytes.
    ///
    /// This function will consume the provided data and use the underlying
    /// bytes to construct a new string, ensuring that there is a trailing 0
    /// byte. This trailing 0 byte will be appended by this function; the
    /// provided data should not contain any 0 bytes in it.
    pub fn new<T>(t: T) -> Result<Self, NulError>
    where
        T: Into<Vec<u8>>,
    {
        Ok(Self {
            inner: CString::new(t)?,
        })
    }

    /// Creates a new C-compatible string from a [`CString`].
    pub fn from_c_string(s: CString) -> Self {
        Self { inner: s }
    }

    /// Unwrap the [`CString`].
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
