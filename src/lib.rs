extern crate rand;
extern crate sha2;
#[cfg(feature="testing")]
#[macro_use]
extern crate serde_derive;

mod r3;
mod rq;
mod zx;

use sha2::{Sha512, Digest};

const PK_SIZE: usize = 1218; // Public Key
const SK_SIZE: usize = 1600; // Private/Secret Key
const CT_SIZE: usize = 1047; // Cipher Text
const K_SIZE: usize = 32;    // Shared Key

fn derive_key(f: [i8; 761], g: [i8;761], gr: [i8;761])-> ([u8; PK_SIZE], [u8; SK_SIZE]){
    let f3r = rq::reciprocal3(f);
    let mut h = [0i16; 761];
    rq::mult(&mut h, f3r, g);
    let pk = rq::encoding::encode(h);
    let mut sk = [0u8; SK_SIZE];
    sk[..191].copy_from_slice(&zx::encoding::encode(f));
    sk[191..382].copy_from_slice(&zx::encoding::encode(gr));
    sk[382..].copy_from_slice(&pk);
    (pk, sk)
}

pub fn generate_key()->([u8; PK_SIZE], [u8; SK_SIZE]){
    let mut rng = rand::thread_rng();
    let mut g = [0i8; 761];
    let gr = loop {
        zx::random::random_small(&mut g, &mut rng);
        let (mask, gr) = r3::reciprocal(g);
        if mask == 0{
            break gr;
        }
    };
    let mut f = [0i8; 761];
    zx::random::random_tsmall(&mut f, &mut rng);
    derive_key(f, g, gr)
}

fn create_cipher(r: [i8; 761], pk :[u8; PK_SIZE])-> 
    ([u8; CT_SIZE], [u8; K_SIZE]){
    let h = rq::encoding::decode(&pk);
    let mut c = [0i16; 761];
    rq::mult(&mut c, h ,r);
    rq::round3(&mut c);
    let mut k = [0u8; 32];
    let s = Sha512::digest(&zx::encoding::encode(r));
    k.copy_from_slice(&s[32..]);
    let mut cstr = [0u8; 1047];
    cstr[..32].copy_from_slice(&s[..32]);
    cstr[32..].copy_from_slice(&rq::encoding::encode_rounded(c));
    (cstr, k)
}

pub fn encapsulate(pk : [u8; PK_SIZE])-> ([u8; CT_SIZE], [u8; K_SIZE]){
    let mut r = [0i8; 761];
    let mut rng = rand::thread_rng();
    zx::random::random_tsmall(&mut r, &mut rng);   
    create_cipher(r, pk)
}

pub fn decapsulate(cstr: [u8; CT_SIZE], sk: [u8; SK_SIZE])-> ([u8; K_SIZE], bool){
    let f = zx::encoding::decode(&sk[..191]);
    let c = rq::encoding::decode_rounded(&cstr[32..]);
    let mut t = [0i16; 761];
    rq::mult(&mut t, c ,f);
    let mut t3 = [0i8;761];
    for i in 0..761{
        t3[i] = r3::mod3::freeze(rq::modq::freeze(3 * t[i] as i32) as i32);
    }
    let gr = zx::encoding::decode(&sk[191..]);
    let mut r = [0i8; 761];
    r3::mult(&mut r, t3, gr);
    let mut w = 0;
    // todo rust-const-time
    for i in 0..761{
        if r[i] != 0{
            w += 1
        }
    }
    let mut ok = w == 286;
    let h = rq::encoding::decode(&sk[(2 * 191)..]);
    let mut hr = [0i16; 761];
    rq::mult(&mut hr, h, r);
    rq::round3(&mut hr);
    for i in 0..761{
        ok &= (hr[i] - c[i]) == 0;
    }
    let s = Sha512::digest(&zx::encoding::encode(r));
    ok &= s[..32] == cstr[..32];
    let mut k = [0u8; 32];
    k.copy_from_slice(&s[32..]);
    (k, ok)
}

#[cfg(test)]
#[cfg(feature="testing")]
mod tests {
    extern crate serde_json;
    extern crate hex;

    use std::fs::File;
    use super::*;


    #[derive(Deserialize)]
    struct KAT {
        c: String,
        k: String,
        pk: String,
        sk: String
    }

    fn parse_kat_file()-> Vec<KAT>{
        let f = File::open("src/tests/kat.json").expect("kat.json not found");
        serde_json::from_reader(f).expect("Error reading kat.json")
    }

    #[test]
    fn decap_kats(){
        let kats = parse_kat_file();
        for (i, kat) in kats.into_iter().enumerate(){
            let ct = ct_to_arr(&kat.c);
            let sk = sk_to_arr(&kat.sk);
            let (k, _) = decapsulate(ct, sk);
            println!("Decap KAT #: {}", i+1);
            println!("c: {}\n", kat.c);
            println!("sk: {}\n", kat.sk);
            println!("expected k: {}", kat.k);
            println!("decapped k: {}\n", hex::encode(k).to_uppercase());
            assert_eq!(k , k_to_arr(&kat.k));
        }
    }

    #[test]
    fn encap_kats(){
        let kats = parse_kat_file();
        for (i, kat) in kats.into_iter().enumerate(){
            let pk = pk_to_arr(&kat.pk); 
            let (ct, k) = encapsulate(pk);
            let sk = sk_to_arr(&kat.sk); 
            let (expected_k, _) = decapsulate(ct, sk);
            println!("Encap KAT #: {}", i+1);
            println!("encapped k: {}", hex::encode(k));
            println!("expected k: {}\n", hex::encode(expected_k));
            assert_eq!(expected_k, k)
        }
    }

    #[test]
    fn keygentest() {
        for _ in 0..5{
            let (pk, sk) = generate_key();
            let (c, k) = encapsulate(pk);
            let (result, _) = decapsulate(c, sk);
            assert_eq!(result, k);
        }
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

    fn k_to_arr(s: &str)-> [u8; K_SIZE]{
        let mut arr = [0u8; K_SIZE];
        arr.copy_from_slice(&hex::decode(s).unwrap()[..]);
        arr
    }

    fn pk_to_arr(s: &str)-> [u8; PK_SIZE]{
        let mut arr = [0u8; PK_SIZE];
        arr.copy_from_slice(&hex::decode(s).unwrap()[..]);
        arr
    }
}
