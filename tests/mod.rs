#[macro_use]
#[cfg(feature="tests")]
extern crate serde_derive;

#[cfg(test)]
#[cfg(feature="tests")]
pub mod tests{
    extern crate serde_json;
    extern crate hex;
    extern crate streamlined_ntru_prime as sntrup;

    use self::sntrup::*;
    use std::fs::File;
    
    #[derive(Deserialize)]
    struct KAT {
        c: String,
        k: String,
        pk: String,
        sk: String
    }

    fn parse_kat_file()-> Vec<KAT>{
        let path = "tests/kat.json";
        let f = File::open(path).expect(&format!("kat.json not found: {}", path));
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
