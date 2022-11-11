use rand::RngCore;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

pub fn gen_u128_from_buf(buf: &[u8]) -> u128 {
    let mut rdr = Cursor::new(buf);

    rdr.read_u128::<BigEndian>().expect("Could not generate i64")
}

pub fn generate_rand_num() -> u128 {
    let mut buf= [0; 32];

    rand::thread_rng().fill_bytes(&mut buf);

    gen_u128_from_buf(&buf)
}