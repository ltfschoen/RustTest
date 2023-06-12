

// use ethereum_types::H256;
// use keccak_hash::keccak;
use std::error::Error;
use bytes::Bytes;
use hex::{decode, FromHex};
use std::str::from_utf8_unchecked;
use std::string::String;

pub fn last_bytes(slice: &[u8]) -> &[u8] {
    slice.split_at(slice.len() - 4).1
}

fn main() -> Result<(), Box<dyn Error>> {
    // note: Hash is [u8; 32] 32 bytes
    
    let block_hash_entropy: &[u8] = "aef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea".as_bytes();
    assert!(block_hash_entropy.len() == 64, "block hash should be 256 bit block hash");
    println!("block_hash_entropy: {:?}\n", block_hash_entropy);
    // https://peterlyons.com/problog/2017/12/rust-converting-bytes-chars-and-strings/
    let array_of_u8 = last_bytes(&block_hash_entropy);
    println!("array_of_u8: {:?}\n", array_of_u8);
    let raw = String::from_utf8_lossy(&array_of_u8);
    println!("raw: {:?}", raw);
    // use u16 since max value 65535
    // let without_prefix = raw.trim_start_matches("0x");
    let decimal = i16::from_str_radix(&raw, 16);
    println!("decimal {:?}", decimal);

    // println!("block_hash_entropy: {:?}\n", block_hash_entropy);
    // let hash = decode(block_hash_entropy);
    // println!("hash: {:?}\n", hash);
    // let buffer: &[u8] = &<[u8; 32]>::from_hex(block_hash_entropy)?;
    // println!("buffer: {:?}\n", buffer);

    // // str::from_utf8(b) from Bytes
    // let entropy: &str = std::str::from_utf8(&buffer)?;//.expect("invalid buffer length");
    // println!("entropy: {:?}\n", entropy.to_string());
    // // get last 5 chars ascii chars that are one byte each
    // let len = entropy.len();
    // let sub_entropy = &entropy[len-5..];
    // println!("sub_entropy: {:?}\n", sub_entropy);

    // let z: &[u8] = &[b'a', b'b'];
    // println!("z {:?}", z);
    // let stack_str: &str = std::str::from_utf8(z).unwrap();
    // println!("stack_str is: {}", stack_str.to_string());
    Ok(())
}
