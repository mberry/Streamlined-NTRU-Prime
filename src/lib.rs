extern crate rand;
extern crate sha2;

mod r3;
mod rq;
mod zx;

use sha2::{Sha512, Digest};

pub const PK_SIZE: usize = 1218; // Public Key
pub const SK_SIZE: usize = 1600; // Private/Secret Key
pub const CT_SIZE: usize = 1047; // Cipher Text
pub const K_SIZE: usize = 32;    // Shared Key

pub const P : usize = 761;
pub const Q : usize = 4591;
pub const W: usize = 286;

///Generates a public and private keypair
/// # Example
/// ```
/// let (public_key, private_key) = generate_key();
/// ```
pub fn generate_key()->([u8; PK_SIZE], [u8; SK_SIZE]){
    let mut rng = rand::thread_rng();
    let mut g = [0i8; P];
    let gr = loop {
        zx::random::random_small(&mut g, &mut rng);
        let (mask, gr) = r3::reciprocal(g);
        if mask == 0{
            break gr;
        }
    };
    let mut f = [0i8; P];
    zx::random::random_tsmall(&mut f, &mut rng);
    derive_key(f, g, gr)
}

/// Encapsulates a public key.
/// Returns a ciphertext and shared key
/// # Example
/// ```
/// let (cipher_text, shared_secret) = encapsulate(public_key);
/// ```
pub fn encapsulate(pk : [u8; PK_SIZE])-> ([u8; CT_SIZE], [u8; K_SIZE]){
    let mut r = [0i8; P];
    let mut rng = rand::thread_rng();
    zx::random::random_tsmall(&mut r, &mut rng);   
    create_cipher(r, pk)
}

/// Decapsulates ciphertext with a known private key.
/// Returns a result containing the shared key.
/// # Example
/// ```
/// let shared_secret = decapsulate(cipher_text, private_key).expect("Decapsulation error") 
/// ```
pub fn decapsulate(cstr: [u8; CT_SIZE], sk: [u8; SK_SIZE])-> Result<[u8; K_SIZE], bool>{
    let f = zx::encoding::decode(&sk[..191]);
    let c = rq::encoding::decode_rounded(&cstr[32..]);
    let mut t = [0i16; P];
    rq::mult(&mut t, c ,f);
    let mut t3 = [0i8;P];
    for i in 0..P{
        t3[i] = r3::mod3::freeze(rq::modq::freeze(3 * t[i] as i32) as i32);
    }
    let gr = zx::encoding::decode(&sk[191..]);
    let mut r = [0i8; P];
    r3::mult(&mut r, t3, gr);
    let w = count_zeroes(r);
    let mut check = w == 286;
    let h = rq::encoding::decode(&sk[(2 * 191)..]);
    let mut hr = [0i16; P];
    rq::mult(&mut hr, h, r);
    rq::round3(&mut hr);
    for i in 0..P{
        check &= (hr[i] - c[i]) == 0;
    }
    let s = Sha512::digest(&zx::encoding::encode(r));
    check &= s[..32] == cstr[..32];
    let mut k = [0u8; 32];
    k.copy_from_slice(&s[32..]);
    if check { Ok(k) } else { Err(false) }
}

fn count_zeroes(r: [i8; P])-> i32{
    let mut w: i32 = 0;
    for i in r.iter(){
        w += i.abs() as i32;
    }
    w
}

fn derive_key(f: [i8; P], g: [i8;P], gr: [i8;P])-> ([u8; PK_SIZE], [u8; SK_SIZE]){
    let f3r = rq::reciprocal3(f);
    let mut h = [0i16; P];
    rq::mult(&mut h, f3r, g);
    let pk = rq::encoding::encode(h);
    let mut sk = [0u8; SK_SIZE];
    sk[..191].copy_from_slice(&zx::encoding::encode(f));
    sk[191..382].copy_from_slice(&zx::encoding::encode(gr));
    sk[382..].copy_from_slice(&pk);
    (pk, sk)
}

fn create_cipher(r: [i8; P], pk :[u8; PK_SIZE])-> 
    ([u8; CT_SIZE], [u8; K_SIZE]){
    let h = rq::encoding::decode(&pk);
    let mut c = [0i16; P];
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