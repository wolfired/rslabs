#![allow(unused_imports)]
#![allow(dead_code)]

use std::default::Default;
use std::io::{Cursor, Read, Seek, SeekFrom};

use rslabs::adler32::adler32;

fn main() {
    {
        println!("0x{:x}", adler32("abcdefghij".as_bytes()));
    }
    {
        enum Endian {
            Big,
            Little,
        }

        fn read_u16<T: Read>(reader: &mut T) -> u16 {
            let mut buf = [0u8; 2];
            let _ = reader.read(&mut buf);
            ((buf[1] as u16) << 8) + buf[0] as u16
        }

        let raw = [0x12, 0x34, 2, 3, 4, 5, 6, 7];
        let mut cr = Cursor::new(raw);
        let v = read_u16(&mut cr);
        println!("0x{:x}", v);
        let _ = cr.seek(SeekFrom::Current(2));
        for b in cr.bytes() {
            println!("{}", b.unwrap());
        }
    }

    // let b = include_bytes!("/home/link/download/Boo.ico");
    // println!("0x{:x}", ((b[1] as u32) << 8) + (b[0] as u32));
    // println!("0x{:x}", ((b[3] as u32) << 8) + (b[2] as u32));

    // let count = ((b[5] as u32) << 8) + (b[4] as u32);
    // println!("0x{:x}", count);

    // for i in 0..count {
    //     let base = (6 + i * 16) as usize;
    //     let wid = if 0 == b[base + 0] {
    //         256
    //     } else {
    //         b[base + 0] as u32
    //     };
    //     let hei = if 0 == b[base + 1] {
    //         256
    //     } else {
    //         b[base + 1] as u32
    //     };
    //     let color_num = b[base + 2] as u32;

    //     let color_planes = ((b[base + 5] as u32) << 8) + (b[base + 4] as u32);
    //     let pixel_per_bit = ((b[base + 7] as u32) << 8) + (b[base + 6] as u32);

    //     let size_bytes = ((b[base + 11] as u32) << 24)
    //         + ((b[base + 10] as u32) << 16)
    //         + ((b[base + 9] as u32) << 8)
    //         + ((b[base + 8] as u32) << 0);

    //     let offset = ((b[base + 15] as u32) << 24)
    //         + ((b[base + 14] as u32) << 16)
    //         + ((b[base + 13] as u32) << 8)
    //         + ((b[base + 12] as u32) << 0);

    //     println!(
    //         "wid = {}, hei = {}, color num = {}, color planes = {}, bits per pixel = {}, size bytes = {}, offset = {}",
    //         wid, hei, color_num, color_planes, pixel_per_bit, size_bytes, offset,
    //     );
    // }
}
