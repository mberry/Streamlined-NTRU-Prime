#![feature(test)]

#[macro_use]
#[cfg(feature="testing")]
extern crate serde_derive;
#[cfg(feature="testing")]
extern crate serde_json;
extern crate hex;
extern crate test;
extern crate streamlined_ntru_prime as sntrup;

#[cfg(feature="testing")]
use test::Bencher;
use self::sntrup::*;
use std::fs::File;

#[bench]
fn key_gen_bench(b: &mut Bencher){
    b.iter(|| generate_key());
}

#[bench]
fn encapsulate_bench(b: &mut Bencher){
    let kat = &parse_kat_file()[0];
    b.iter(|| encapsulate(pk_to_arr(&kat.pk)));
}

#[bench]
fn decapsulate_bench(b: &mut Bencher){
    let kat = &parse_kat_file()[2];
    b.iter(|| decapsulate(ct_to_arr(&kat.c), sk_to_arr(&kat.sk)));
}

fn parse_kat_file()-> Vec<KAT>{
    let path = "tests/kat.json";
    let f = File::open(path).expect(&format!("kat.json not found: {}", path));
    serde_json::from_reader(f).expect("Error reading kat.json")
}

#[derive(Deserialize)]
struct KAT {
    c: String,
    pk: String,
    sk: String
}

fn ct_to_arr(s: &str)-> [u8; CT_SIZE]{
    let mut arr = [0u8; CT_SIZE];
    arr.copy_from_slice(&hex::decode(s).unwrap()[..]);
    arr
}

fn sk_to_arr(s: &str)-> [u8; SK_SIZE]{
    let mut arr = [0u8; SK_SIZE];
    arr.copy_from_slice(&hex::decode(s).unwrap()[..]);
    arr
}

fn pk_to_arr(s: &str)-> [u8; PK_SIZE]{
    let mut arr = [0u8; PK_SIZE];
    arr.copy_from_slice(&hex::decode(s).unwrap()[..]);
    arr
}