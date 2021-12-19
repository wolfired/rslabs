#![allow(unused_variables)]
#![allow(dead_code)]

use std::{collections::HashMap, str::from_utf8};

#[derive(Debug)]
pub enum BTType {
    BTBytes(Vec<u8>),
    BTInteger(i32),
    BTList(Vec<BTType>),
    BTDict(HashMap<String, BTType>),
}

impl BTType {
    const FLAG_0: u8 = b'0';
    const FLAG_9: u8 = b'9';
    const FLAG_I: u8 = b'i';
    const FLAG_L: u8 = b'l';
    const FLAG_D: u8 = b'd';
    const FLAG_S: u8 = b':';
    const FLAG_E: u8 = b'e';

    pub fn from_raw(bytes: &[u8]) -> Option<Self> {
        let mut index = 0;
        let mut pairs: Vec<(usize, u8)> = Vec::new();

        while index < bytes.len() {
            let mut shift = 1;
            match bytes[index] {
                byte @ Self::FLAG_0..=Self::FLAG_9 => {
                    // if pairs.is_empty() || Self::FLAG_I != pairs.last().unwrap().1 {
                    //     pairs.push((index, byte));
                    // }
                    shift += Self::decode_bytes(&bytes[index..]);
                }
                byte @ Self::FLAG_I => {
                    pairs.push((index, byte));
                }
                byte @ Self::FLAG_L => {
                    pairs.push((index, byte));
                }
                byte @ Self::FLAG_D => {
                    pairs.push((index, byte));
                }
                byte @ Self::FLAG_S => {
                    let mut exp = 0;
                    while !pairs.is_empty()
                        && Self::FLAG_0 <= pairs.last().unwrap().1
                        && pairs.last().unwrap().1 <= Self::FLAG_9
                    {
                        let pair = pairs.pop().unwrap();

                        shift += (pair.1 as usize - 0x30) * 10_usize.pow(exp);

                        exp += 1;
                    }
                    pairs.push((index, byte));
                    pairs.push((index + shift, Self::FLAG_E));
                }
                byte @ Self::FLAG_E => {
                    pairs.push((index, byte));
                }
                _ => {}
            }
            index += shift;
        }

        if pairs.is_empty() {
            return None;
        }

        // for (i, &(j, v)) in pairs.iter().enumerate() {
        //     println!("{:08} -> {:08} -> {}", i, j, char::from_u32(v as u32).unwrap());
        // }

        // loop {
        //     let mut index = 0;
        //     let mut shift = 0;
        //     match pairs[index] {
        //         (cursor, Self::FLAG_S) => {
        //             shift = Self::decode_bytes();
        //         }
        //         (cursor, Self::FLAG_I) => {
        //             shift = Self::decode_integer();
        //         }
        //         _ => break,
        //     }
        //     index += shift;
        // }

        let mut index = 0;
        while index < pairs.len() {
            index += 1;
        }

        return Some(Self::BTInteger(0));
    }

    fn decode_bytes(bytes: &[u8]) -> usize {
        println!("decode_bytes");

        let mut i = 0;

        while i < bytes.len() {
            if Self::FLAG_S == bytes[i] {
                let bytes_count = from_utf8(&bytes[..i]).unwrap().parse::<usize>().unwrap();
                println!("{}", bytes_count);
                i += bytes_count;
                break;
            }else{
                i += 1;
            }
        }

        // for i in ..bytes.len() {
            
        // }

        i
    }

    fn decode_integer() -> usize {
        println!("decode_integer");
        0
    }
}
