// Copyright (c) 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::ffi::CString;

use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};

/// Time data for *scheduled_delivery_time*, *validity_period*, and
/// *final_date*.
#[derive(Clone, Debug)]
pub enum Time {
    Null,
    Absolute(DateTime<FixedOffset>),
    Relative(Duration),
}

impl bincode::Decode for Time {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        use super::COctet;

        let s = COctet::decode(decoder)?.as_c_string().into_bytes();
        if s.is_empty() {
            return Ok(Self::Null);
        }

        if s.len() != 16 {
            return Err(bincode::error::DecodeError::OutsideUsizeRange(17));
        }

        match &s[15] {
            #[allow(non_snake_case)]
            &b'R' => {
                let YY = (s[0] - b'0') * 10 + s[1] - b'0';
                let MM = (s[2] - b'0') * 10 + s[3] - b'0';
                let DD = (s[4] - b'0') * 10 + s[5] - b'0';
                let hh = (s[6] - b'0') * 10 + s[7] - b'0';
                let mm = (s[8] - b'0') * 10 + s[9] - b'0';
                let ss = (s[10] - b'0') * 10 + s[11] - b'0';
                let period = chrono::Duration::days(i64::from(YY) * 365 + i64::from(MM) * 30 + i64::from(DD))
                    + chrono::Duration::hours(i64::from(hh))
                    + chrono::Duration::minutes(i64::from(mm))
                    + chrono::Duration::seconds(i64::from(ss));
                Ok(Self::Relative(period))
            }

            #[allow(non_snake_case)]
            &b'+' | &b'-' => {
                let YY = (s[0] - b'0') * 10 + s[1] - b'0';
                let YY = i32::from(YY) + 2000;
                let MM = (s[2] - b'0') * 10 + s[3] - b'0';
                let DD = (s[4] - b'0') * 10 + s[5] - b'0';
                let hh = (s[6] - b'0') * 10 + s[7] - b'0';
                let mm = (s[8] - b'0') * 10 + s[9] - b'0';
                let ss = (s[10] - b'0') * 10 + s[11] - b'0';
                let t = u32::from(s[12] - b'0');
                let datetime = NaiveDateTime::new(
                    NaiveDate::from_ymd(YY, u32::from(MM), u32::from(DD)),
                    NaiveTime::from_hms_micro(u32::from(hh), u32::from(mm), u32::from(ss), t * 100),
                );
                let offset = {
                    let h = (s[13] - b'0') * 10 + s[14] - b'0';
                    let secs = i32::from(h) * 15 * 60;
                    if s[15] == b'+' {
                        FixedOffset::east(secs)
                    } else {
                        FixedOffset::west(secs)
                    }
                };
                let datetime = DateTime::<FixedOffset>::from_local(datetime, offset);

                Ok(Self::Absolute(datetime))
            }

            _ => Err(bincode::error::DecodeError::OtherString(
                CString::new(s).unwrap_or_default().into_string().unwrap_or_default(),
            )),
        }
    }
}

impl bincode::Encode for Time {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        use super::COctet;

        let v = match self {
            Self::Null => COctet::new("").unwrap(),
            Self::Absolute(t) => {
                let s = format!("{}", t.format("%y%m%d%H%M%S0%f"));
                let zone = t.offset().local_minus_utc() / 900;
                let (zone, sign) = match zone {
                    z if z > 48 => (96 - z, -1),
                    z if z < -48 => (z + 96, 1),
                    z => (z.abs(), z.signum()),
                };
                let nnp = format!("{:02}{:+2}", zone, sign);
                let mut v = s.as_bytes()[..13].to_vec();
                v.extend_from_slice(&nnp.as_bytes()[..3]);
                COctet::new(v).unwrap()
            }
            #[allow(non_snake_case)]
            Self::Relative(t) => {
                let ss = t.num_seconds();
                let (DD, ss) = (ss / 86400, ss % 86400);
                let (YY, DD) = (DD / 365 % 100, DD % 365);
                let (MM, DD) = (DD / 30, DD % 30);
                let (hh, ss) = (ss / 3600, ss % 3600);
                let (mm, ss) = (ss / 60, ss % 60);

                let s = format!("{:02}{:02}{:02}{:02}{:02}{:02}000R", YY, MM, DD, hh, mm, ss);

                COctet::new(s).unwrap()
            }
        };

        v.encode(encoder)
    }
}
