//extern crate lz4;
extern crate lz4_compress;
extern crate byteorder;

use std::str;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use byteorder::{LittleEndian, ReadBytesExt};

const MAGIC_HEADER : &[u8] = b"mozLz40\0";

fn unpack(path: &str) {
    let f = File::open(path).expect("Can't open file");
    let mut reader = f;

    let mut buf = [0; 8];
    reader.read_exact(&mut buf).expect("Can't read header");

    if buf != MAGIC_HEADER {
        panic!("Wrong magic header");
    }

    let outsize = reader.read_u32::<LittleEndian>().expect("Can't read output size");

    let mut data = Vec::with_capacity(outsize as usize);
    reader.read_to_end(&mut data).expect("Can't read file into memory");

    let decompressed = lz4_compress::decompress(&data).expect("can't decode data");

    let mut out = io::stdout();
    out.write_all(&decompressed).expect("Can't write decompressed data to stdout");

    if decompressed.len() != outsize as usize {
        eprintln!("Length mismatch. Expected {}, got {}", outsize, decompressed.len());
    }
}

fn main() {
    let args = env::args().skip(1);

    for arg in args {
        eprintln!("Unpacking {:?}", arg);

        unpack(&arg);
    }
}
