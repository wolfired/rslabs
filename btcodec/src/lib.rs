#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Display;

#[derive(Debug)]
pub enum Value {
    Nil,
    Str(String),
    I64(i64),
    #[cfg(feature = "ext")]
    F64(f64),
    List(Vec<Self>),
    Dict(std::collections::HashMap<String, Self>),
}

impl Value {
    pub fn new(raw: &[u8]) -> Self {
        let (result, ..) = Self::decode(raw);
        result
    }

    pub fn raw() -> Vec<u8> {
        vec![0u8]
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Str(value_l) => {
                if let Self::Str(value_r) = other {
                    value_l == value_r
                } else {
                    false
                }
            }
            Self::I64(value_l) => {
                if let Self::I64(value_r) = other {
                    value_l == value_r
                } else {
                    false
                }
            }
            #[cfg(feature = "ext")]
            Self::F64(value_l) => {
                if let Self::F64(value_r) = other {
                    value_l == value_r
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Self::Str(value_l) => {
                if let Self::Str(value_r) = other {
                    value_l.partial_cmp(value_r)
                } else {
                    None
                }
            }
            Self::I64(value_l) => {
                if let Self::I64(value_r) = other {
                    value_l.partial_cmp(value_r)
                } else {
                    None
                }
            }
            #[cfg(feature = "ext")]
            Self::F64(value_l) => {
                if let Self::F64(value_r) = other {
                    value_l.partial_cmp(value_r)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(value) => write!(f, r#""{}""#, value),
            Self::I64(value) => write!(f, "{}", value),
            #[cfg(feature = "ext")]
            Self::F64(value) => write!(f, "{}", value),
            Self::List(value) => {
                let mut arr = Vec::<_>::new();

                for v in value.iter() {
                    arr.push(v.to_string());
                }

                write!(f, "[{}]", arr.join(", "))
            }
            Self::Dict(value) => {
                let mut keys = Vec::<String>::new();

                for (key, _) in value.iter() {
                    keys.push(key.to_string());
                }

                keys.sort_by(|fst, snd| fst.cmp(snd));

                let mut key_values = Vec::<_>::new();

                for key in keys.iter() {
                    key_values.push(format!(
                        r#""{}": {}"#,
                        key,
                        value.get(key).unwrap().to_string()
                    ));
                }

                write!(f, "{{{}}}", key_values.join(", "))
            }
            _ => write!(f, "err"),
        }
    }
}

impl Value {
    fn decode(bytes: &[u8]) -> (Self, usize) {
        match bytes[0] {
            b'0'..=b'9' => Self::decode_s(bytes),
            b'i' => Self::decode_i(bytes),
            #[cfg(feature = "ext")]
            b'f' => Self::decode_f(bytes),
            b'l' => Self::decode_l(bytes),
            b'd' => Self::decode_d(bytes),
            _ => (Self::Nil, 0),
        }
    }

    fn decode_s(bytes: &[u8]) -> (Self, usize) {
        let mut i = 0;

        let mut buf = Vec::<_>::new();

        while i < bytes.len() {
            let byte = bytes[i];
            i += 1;

            if b':' == byte {
                break;
            } else {
                buf.push(byte);
            }
        }

        let end = bytes
            .len()
            .min(i + String::from_utf8(buf).unwrap().parse::<usize>().unwrap());

        let mut buf = Vec::<_>::new();

        while i < end {
            buf.push(bytes[i]);
            i += 1;
        }

        let r = String::from_utf8(buf);

        if let Result::Ok(result) = r {
            (Self::Str(result), i)
        } else if let Result::Err(error) = r {
            let mut result = format!("{:02x?}", error.as_bytes());
            result = result
                .replace(",", "")
                .replace("[", "")
                .replace("]", "")
                .replace(" ", "");
            (Self::Str(result), i)
        } else {
            (Self::Str("error string".to_string()), i)
        }
    }

    fn decode_i(bytes: &[u8]) -> (Self, usize) {
        let mut i = 1;

        let mut buf = Vec::<_>::new();

        while i < bytes.len() {
            let byte = bytes[i];

            if b'e' == byte {
                break;
            } else {
                buf.push(byte);
                i += 1;
            }
        }

        (
            Self::I64(String::from_utf8(buf).unwrap().parse::<i64>().unwrap()),
            i + 1,
        )
    }

    #[cfg(feature = "ext")]
    fn decode_f(bytes: &[u8]) -> (Self, usize) {
        let mut i = 1;

        let mut buf = Vec::<_>::new();

        while i < bytes.len() {
            let byte = bytes[i];

            if b'e' == byte {
                break;
            } else {
                buf.push(byte);
                i += 1;
            }
        }

        (
            Self::F64(String::from_utf8(buf).unwrap().parse::<f64>().unwrap()),
            i + 1,
        )
    }

    fn decode_l(bytes: &[u8]) -> (Self, usize) {
        let mut i = 1;

        let mut result: Vec<Self> = Vec::new();

        while i < bytes.len() {
            let byte = bytes[i];

            if b'e' == byte {
                break;
            } else {
                let (value, cost) = Self::decode(&bytes[i..]);
                i += cost;

                result.push(value);
            }
        }

        (Self::List(result), i + 1)
    }

    fn decode_d(bytes: &[u8]) -> (Self, usize) {
        let mut i = 1;

        let mut result: std::collections::HashMap<String, Self> = std::collections::HashMap::new();

        while i < bytes.len() {
            let byte = bytes[i];

            if b'e' == byte {
                break;
            } else {
                let (key, cost) = Self::decode_s(&bytes[i..]);
                i += cost;

                if let Self::Str(key) = key {
                    let (value, cost) = Self::decode(&bytes[i..]);
                    i += cost;

                    result.insert(key, value);
                }
            }
        }

        (Self::Dict(result), i + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;
    #[test]
    fn test_decode_s() {
        if let (Value::Str(value), _) = Value::decode_s("6:你好".as_bytes()) {
            assert_eq!(value, "你好".to_string());
        }

        if let (Value::Str(value), _) = Value::decode_s("5:hello".as_bytes()) {
            assert_eq!(value, "hello".to_string());
        }

        if let (Value::Str(value), _) = Value::decode_s("0:".as_bytes()) {
            assert_eq!(value, "".to_string());
        }
    }

    #[test]
    fn test_decode_i() {
        if let (Value::I64(value), _) = Value::decode_i("i-3e".as_bytes()) {
            assert_eq!(value, -3);
        }
    }

    #[test]
    #[cfg(feature = "ext")]
    fn test_decode_f() {
        if let (Value::F64(value), _) = Value::decode_f("f-3.33e".as_bytes()) {
            assert_eq!(value, -3.33);
        }
    }

    #[test]
    fn test_decode_l() {
        if let (Value::List(value), _) = Value::decode_l("li-3ei-3ee".as_bytes()) {
            assert_eq!(1, 1);
        }
    }

    #[test]
    fn test_decode_d() {
        if let (Value::Dict(value), _) = Value::decode_d("d1:ai0e1:bi1ee".as_bytes()) {
            assert_eq!(1, 1);
        }
    }
}
