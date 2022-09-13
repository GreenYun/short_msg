//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

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
            return Ok(Time::Null);
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
                let period = chrono::Duration::days(YY as i64 * 365 + MM as i64 * 30 + DD as i64)
                    + chrono::Duration::hours(hh as i64)
                    + chrono::Duration::minutes(mm as i64)
                    + chrono::Duration::seconds(ss as i64);
                Ok(Time::Relative(period))
            }

            #[allow(non_snake_case)]
            &b'+' | &b'-' => {
                let YY = (s[0] - b'0') * 10 + s[1] - b'0';
                let YY = YY as i32 + 2000;
                let MM = (s[2] - b'0') * 10 + s[3] - b'0';
                let DD = (s[4] - b'0') * 10 + s[5] - b'0';
                let hh = (s[6] - b'0') * 10 + s[7] - b'0';
                let mm = (s[8] - b'0') * 10 + s[9] - b'0';
                let ss = (s[10] - b'0') * 10 + s[11] - b'0';
                let t = (s[12] - b'0') as u32;
                let datetime = NaiveDateTime::new(
                    NaiveDate::from_ymd(YY, MM as u32, DD as u32),
                    NaiveTime::from_hms_micro(hh as u32, mm as u32, ss as u32, t * 100),
                );
                let offset = {
                    let h = (s[13] - b'0') * 10 + s[14] - b'0';
                    let secs = (h as i32) * 15 * 60;
                    if s[15] == b'+' {
                        FixedOffset::east(secs)
                    } else {
                        FixedOffset::west(secs)
                    }
                };
                let datetime = DateTime::<FixedOffset>::from_local(datetime, offset);

                Ok(Time::Absolute(datetime))
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
            Time::Null => COctet::new("").unwrap(),
            Time::Absolute(t) => {
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
            Time::Relative(t) => {
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

#[cfg(test)]
mod test {
    use chrono::{FixedOffset, TimeZone, Utc};

    use super::Time;

    #[test]
    fn test() {
        let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();
        let epoch = Utc.timestamp(0, 0);
        let now = Utc::now();
        let t = Time::Null;
        let v = bincode::encode_to_vec(t, config).unwrap();
        println!("{:x?}", v);
        let (d, _) = bincode::decode_from_slice::<Time, _>(&v, config).unwrap();
        println!("{:?}", d);
        let t = Time::Absolute(now.with_timezone(&FixedOffset::east(0)));
        let v = bincode::encode_to_vec(t, config).unwrap();
        println!("{:x?}", v);
        let (d, _) = bincode::decode_from_slice::<Time, _>(&v, config).unwrap();
        println!("{:?}", d);
        let t = Time::Absolute(now.with_timezone(&FixedOffset::east(8 * 3600)));
        let v = bincode::encode_to_vec(t, config).unwrap();
        println!("{:x?}", v);
        let (d, _) = bincode::decode_from_slice::<Time, _>(&v, config).unwrap();
        println!("{:?}", d);
        let t = Time::Absolute(now.with_timezone(&FixedOffset::east(-8 * 3600)));
        let v = bincode::encode_to_vec(t, config).unwrap();
        println!("{:x?}", v);
        let (d, _) = bincode::decode_from_slice::<Time, _>(&v, config).unwrap();
        println!("{:?}", d);
        let t = Time::Relative(now - epoch);
        let v = bincode::encode_to_vec(t, config).unwrap();
        println!("{:x?}", v);
        let (d, _) = bincode::decode_from_slice::<Time, _>(&v, config).unwrap();
        println!("{:?}", d);
    }
}
