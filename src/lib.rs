
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate sha2;

pub use sha2::{Sha512, Digest};

pub mod r3;
pub mod rq;
pub mod zx;

pub use rq::encoding;
pub use r3::mod3;
use rq::*;


const PUBLIC_KEY_SIZE: usize = 1218;
const PRIVATE_KEY_SIZE: usize = 1600;
const CIPHER_TEXT_SIZE: usize = 1047;
const SHARED_KEY_SIZE: usize = 32;

pub fn derive_key(f: [i8; 761], g: [i8;761], gr: [i8;761])-> ([u8; PUBLIC_KEY_SIZE], [u8; PRIVATE_KEY_SIZE]){
    let f3r = [0i16; 761];
    rq::reciprocal3(f3r, f);
    let mut h = [0i16; 761];
    rq::mult(&mut h, f3r, g);
    let pk = rq::encoding::encode(h);

    let mut sk = [0u8; PRIVATE_KEY_SIZE];
    sk[..191].copy_from_slice(&zx::encoding::encode(f));
    sk[191..382].copy_from_slice(&zx::encoding::encode(gr));
    sk[382..].copy_from_slice(&pk);

    (pk, sk)
}

pub fn generate_key()->([u8; PUBLIC_KEY_SIZE], [u8; PRIVATE_KEY_SIZE]){
    let mut rng = rand::thread_rng();
    let (mut g, mut gr) = ([0i8; 761], [0i8; 761]);
    loop {
        zx::random::random_small(&mut g, &mut rng);
        if r3::reciprocal(gr, g) == 0{
            break;
        }
    }

    let mut f = [0i8; 761];
    zx::random::random_tsmall(&mut f, &mut rng);

    derive_key(f, g, gr)


    //([0u8; PUBLIC_KEY_SIZE], [0u8; PRIVATE_KEY_SIZE])
}

pub fn create_cipher(r: [i8; 761], pk :[u8; PUBLIC_KEY_SIZE])-> 
    ([u8; CIPHER_TEXT_SIZE], [u8; SHARED_KEY_SIZE]){
    
    let h = rq::encoding::decode(&pk);
    let mut c = [0i16; 761];
    rq::mult(&mut c, h ,r);
    rq::round3(&mut c);

    let mut k = [0u8; 32];
    //let mut hasher = new();
    let s = Sha512::digest(&zx::encoding::encode(r));
    k.copy_from_slice(&s[32..]);

    let mut cstr = [0u8; 1047];
    cstr[..32].copy_from_slice(&s[..32]);
    cstr[32..].copy_from_slice(&rq::encoding::encode_rounded(c));

    (cstr, k)
}

pub fn encapsulate(pk : [u8; PUBLIC_KEY_SIZE])-> ([u8; CIPHER_TEXT_SIZE], [u8; SHARED_KEY_SIZE]){

    let mut r = [0i8; 761];
    let mut rng = rand::thread_rng();
    zx::random::random_tsmall(&mut r, &mut rng);
    create_cipher(r, pk)

    //([0u8; CIPHER_TEXT_SIZE], [0u8; SHARED_KEY_SIZE])
}

pub fn decapsulate(cstr: [u8; CIPHER_TEXT_SIZE], sk: [u8; PRIVATE_KEY_SIZE])-> ([u8; SHARED_KEY_SIZE], bool){

    let f = zx::encoding::decode(&sk[..191]);
    let c = rq::encoding::decode_rounded(&cstr[32..]);
    let mut t = [0i16; 761];
    rq::mult(&mut t, c ,f);

    let mut t3 = [0i8;761];
    for i in 0..761{
        t3[i] = mod3::freeze(modq::freeze(3 * t[i] as i32) as i32);
    }

    let gr = zx::encoding::decode(&sk[191..]);
    let mut r = [0i8; 761];
    r3::mult(&mut r, t3, gr);

    let mut w = 0;
    // todo rust-const-time
    for i in 0..761{
        if r[i] == 0{
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
mod tests {
    extern crate hex;
    extern crate serde;
    extern crate serde_json;
    use super::*;
    use std::fs::File;
    #[test]
    #[ignore]
    fn it_works() {
        let pk_hex = "25C78EE361997051D038B708DBD9554CD84E8815074A7F58AB9E0C5756C7DA4CA46817BB65DFC1258C409E6B1FA14A0EC6EB046E16E1DB1CB60D30332584E2326A27C40CA653B52675607E1AE316853EB7DF6D8DBD96713377A3759842CDE654C501636CBCAEEC3A0EE281A88A2BE22F7178C33F83552B467EEE226C042BC512B1DC4BB83DE4730664FEE1321CE9B057638E0B19B242C31F328BAD91A1EFA94D82863DD002D0FB32824CDA53C345B1498AB428DEEC14F15862285BFC6BBCC318661F616064D9E61F6C7D2B78BBE2090FC45574F8FA545F3B98D8F3FDFC2557499E7D8D19443EC2356049A33E778F8722B2A193FFEB1D354CFF54ED2F6E622F17545B770452FFE31DFF505D69720D1F2B4021EC0415CF1A1273817E641659674D7D891EE4D43BD82F7620319334427B4525CB28AAE9B9543BC689D483BA87F046933AFFD0B4DB7F5611DF74F565FB32224005174CFFAE693B1DB6C31D34E8B6005F5075C446AEDE0BC379F080E6AC92541D425141BCB3A42D3826CBACB191EB1987D8878918C9D233E742CEA71DC1445240F4DBE96624AD4F781C697C20FB6256A4C0EF3AC5233315DF9F236204B0EA1E0380DCFCD1E47D4609573235742A21342ECEA4F8AE18EB01BD15BB7DFA740947B998C66F23F0751C254FA5B96B7B3941E2C5A20610D0B62CA56EA92414F7213970FDFF0761672B02464B63F3B3827611BC2D9C6E9AAABF131D9D2E6862754E466613E0B381F55A163CEAA03637E9B11977EDCAEBF45FFB07A4733ABE329F9D4A6B2797E6BC49242BC8A8FD20F7481C5050D376E363B17514BA4E32FC0AB94227DB7C69F0800FA82AB54664FB9320610B8013E8B091360D1DC1F6702DB749D54CAFD82872DD12D32AA74CED0C2C7EF71899555388CB05885648C8CF19E6BB9B582C9A87AACA5EF021C71776432FD1494C2EF5324AF037530B2971F184749EF74EE2AC800D7455B45A1063D69390F4AD1452C03BB6B8ADC00A809AA00286902654DF380A503A24503BF391039A03C6D81D695A19B381BA175935F0190BA2F36A08300C10C569485F082948EE0E28934C39FFE39D89C6BA92207238E07DF3CFFC3C75F6BF15247F5E171909AD10F8DF6F39B5801B9FF3DDA52D013146DB2AD32A31892D5256E5E099564955C98A54EFEE00DA49E026726C6326E1F99FCF3C00693638878D22C0BC2551C46B761C2C22541F2E19034C8CA1C645AF28A8D0593FF810D8B75EF6954B8E2074819A6C433F6137D1FBC85D4CB5A0593AA7AE0140A9834A3E960D5B5DE3674640C90B7F9F9B10091779AB8D18202B5965C447CB71E29B42BE1C3086C3F57F462A21C45D1EC53D048537C8898D91001B2B068B7A2001C33E7E105B43069B0B38D15ECB3AA6F8403460714FBC93DD961280464DFF96881700A5DFE91B1DE7671FDBDBAA223CA8C2434C5980911185352EB3C98949E416E045783EBC5A715D10134D7E70611B5C87449ABF6831EF630F2D07010848E3BE260A292612D4C2951311AD47CA8C6C6CC424F3EB1643844398339A6894C13280F923E856F9463FCAE3457634060A7D3CED266B1942424F9BAC29F8ABA0081842EF1E0590C546B5AE8B59EC91EDF7DE7B942567F3BC0327319F38AAB453D61817331052B0BD0C352B32507034B63F471ECC4DE9018403122A5635EFFCFAE8CA106F39163586EB97581323AB1CE16A5A382111B607";
        let sk_hex = "1A9421552149194565695555955550A665165401449584815624555165266585154555555618659555556595455659669115894455094414A88565565199461615514599655665152991951955A956645955484591656559660556516699455915A55925599984199455162859555459516605820512465555580915512095954155526669449456696455155656565554691945066555516649554955A5665585514911966A29525619654954654A25545651455615550559156946165502952694921248650696146A8299661000A8800218929A5201814961082441862A80409A854258559155098110008584A68880561921A0150666552116201A226666805A800A16A625480090956A554008856488A8848AA0150416008AAA151404004A6922590265218A9486508525080549A29A4AA09AA8161905046212615581486A698A8AA29110A6A9A98998118A454440485144495A204489041AAA26224416182AA55456529216A1A966849A9512554A806A92611A089A6420A659810125C78EE361997051D038B708DBD9554CD84E8815074A7F58AB9E0C5756C7DA4CA46817BB65DFC1258C409E6B1FA14A0EC6EB046E16E1DB1CB60D30332584E2326A27C40CA653B52675607E1AE316853EB7DF6D8DBD96713377A3759842CDE654C501636CBCAEEC3A0EE281A88A2BE22F7178C33F83552B467EEE226C042BC512B1DC4BB83DE4730664FEE1321CE9B057638E0B19B242C31F328BAD91A1EFA94D82863DD002D0FB32824CDA53C345B1498AB428DEEC14F15862285BFC6BBCC318661F616064D9E61F6C7D2B78BBE2090FC45574F8FA545F3B98D8F3FDFC2557499E7D8D19443EC2356049A33E778F8722B2A193FFEB1D354CFF54ED2F6E622F17545B770452FFE31DFF505D69720D1F2B4021EC0415CF1A1273817E641659674D7D891EE4D43BD82F7620319334427B4525CB28AAE9B9543BC689D483BA87F046933AFFD0B4DB7F5611DF74F565FB32224005174CFFAE693B1DB6C31D34E8B6005F5075C446AEDE0BC379F080E6AC92541D425141BCB3A42D3826CBACB191EB1987D8878918C9D233E742CEA71DC1445240F4DBE96624AD4F781C697C20FB6256A4C0EF3AC5233315DF9F236204B0EA1E0380DCFCD1E47D4609573235742A21342ECEA4F8AE18EB01BD15BB7DFA740947B998C66F23F0751C254FA5B96B7B3941E2C5A20610D0B62CA56EA92414F7213970FDFF0761672B02464B63F3B3827611BC2D9C6E9AAABF131D9D2E6862754E466613E0B381F55A163CEAA03637E9B11977EDCAEBF45FFB07A4733ABE329F9D4A6B2797E6BC49242BC8A8FD20F7481C5050D376E363B17514BA4E32FC0AB94227DB7C69F0800FA82AB54664FB9320610B8013E8B091360D1DC1F6702DB749D54CAFD82872DD12D32AA74CED0C2C7EF71899555388CB05885648C8CF19E6BB9B582C9A87AACA5EF021C71776432FD1494C2EF5324AF037530B2971F184749EF74EE2AC800D7455B45A1063D69390F4AD1452C03BB6B8ADC00A809AA00286902654DF380A503A24503BF391039A03C6D81D695A19B381BA175935F0190BA2F36A08300C10C569485F082948EE0E28934C39FFE39D89C6BA92207238E07DF3CFFC3C75F6BF15247F5E171909AD10F8DF6F39B5801B9FF3DDA52D013146DB2AD32A31892D5256E5E099564955C98A54EFEE00DA49E026726C6326E1F99FCF3C00693638878D22C0BC2551C46B761C2C22541F2E19034C8CA1C645AF28A8D0593FF810D8B75EF6954B8E2074819A6C433F6137D1FBC85D4CB5A0593AA7AE0140A9834A3E960D5B5DE3674640C90B7F9F9B10091779AB8D18202B5965C447CB71E29B42BE1C3086C3F57F462A21C45D1EC53D048537C8898D91001B2B068B7A2001C33E7E105B43069B0B38D15ECB3AA6F8403460714FBC93DD961280464DFF96881700A5DFE91B1DE7671FDBDBAA223CA8C2434C5980911185352EB3C98949E416E045783EBC5A715D10134D7E70611B5C87449ABF6831EF630F2D07010848E3BE260A292612D4C2951311AD47CA8C6C6CC424F3EB1643844398339A6894C13280F923E856F9463FCAE3457634060A7D3CED266B1942424F9BAC29F8ABA0081842EF1E0590C546B5AE8B59EC91EDF7DE7B942567F3BC0327319F38AAB453D61817331052B0BD0C352B32507034B63F471ECC4DE9018403122A5635EFFCFAE8CA106F39163586EB97581323AB1CE16A5A382111B607";
        let ct_hex = "2BB5C65BB619C0CDB038AA3E78B111EF72787919E749E0F86377A70671EA97CECC94425E0113FABE1AF9AB967D32886F5B2FBAD0CE2F84795EA8019D0236C65769FB45C9142FB404265649274DEB50BA3F8A2203D7373604D734F023F5F389C95E35BC9DB2CA2A6A4683432E74A347A232F765476A51BC6554D5BBC190E26A959C49D80843B6A7918F99D470C859B4B074448749CB78601F71246F26D165D8115368EB87AB13FDD20B350BA78093E9CF51CC487893FEE5C3D327B1A0D68D25CEDA24C17220F102C1EB60D7C67D625D3E38AD1A18A33EB424C0C4021A7F8CF7BC31F3C622A5E6EFCCF07D8ECA10F936C92CEEB067889A32798B6D6A263132D78F51EF011373F0F32EBABA453C42F4E1AD106CD377C3CB5E45BFFE481D9BDCCEC5D36A33B719C73A9CD6778686D1EB28D260F5B4A3CAAF46BCBB529BD6F9B5A8B285E00D907A7096C1D9BF8844104DC60F19993705066FDED0803C93063A218B3C909B6BC121B10BC5E5ACA6D1250A314CD5BD17D0EA0ED048556D450C44AA84CD1A41C18E1C1001C4D16F7C188C8B373F71E62DBBA3613657E6F25F1F44430C731999B11253BB36A4A58D5474DCED4BB1DE00925CE1215B502D744C949F6D6A324198AD353BE3ABC2E0E9BD2E5D46B4B3B81495ABA0562D18B30CDED62DF75D49BCCA1CA6EB90C26DDAC602688071E6C72C19B012904EC302011ABA18D25C7D30B8B240A5C8E597175C9135C9181FF181D39DD50F77A945864EC77CD2997F468AA0CCD56752D3E21684C39954EE2968C9E6DF2B7894579233FE007328173619B69BDB181CBCA4EC712E6D2B26A1EB6D893644170AAEC4350775B4C42B3D614C336A84A63EC1A46F176F585A89F00CC41721FE516C2BD314658EED6E40F7B77B79BB0CD5C48D2F737DDE123419B372DE1526E536D5473C4942D54E1C72C893DB515D3596C096754E12D178110392FC37407B1CCC8A19A187351EA63454F6E6EC51F99BF23E7EDBEBB1DDA38750D459992CECCBBEBF41267E21F3D568749A699D180404C7BBD2F739137C7D0324F302C83889DC1997AB1614A67C6D99463F8DB9D00D66457AC0FA13B3BAA5AA8C5329EE9D849DCA4088EE063B90F9E947C4617A4BA33ECC201316D2CE0605882EB95934C585FF399A93A6E4289AACD0AF3FCBF609D41B67182B591E29532E0E590E37E925EDC065C0E7BC69552F97D72DA7FE7A22CCFB1102C40425263613C336B98D3B6982B76349ABC6EE7A7C67C55F90F7444700D53554213647A5EFEEBB1F7FA08D9FFFDBD3CC06EBE083E3C1DC44C597133E79D3A125FD80A734B6721A2C966FB65ADD2866BD8FB5863D8723B4C1E11DF926B1475BD09B5D860411AC190369EDEC4D7645580FFDAF9F47C2110408951DDDB6A79C233C2CEF17BFA69F334DFF4878A725DC23AEACEAB70F92B8E6BCC429A28BCA3662640D4D3D5DE9E4D89555732E44DB6F01BC59A123";
        //let ss_hex = "99AE181100E0DF6392E046F02D5DC4B7DA81647DC3027E6EC68B40C4A5625E6C";
        //let seed_hex = "061550234D158C5EC95595FE04EF7A25767F2E24CC2BC479D09D86DC9ABCFDE7056A8C266F9EF97ED08541DBD2E1FFA1";
        //let pk = hex::decode(pk_hex).unwrap();
        let skvec = hex::decode(sk_hex).unwrap();
        let ctvec = hex::decode(ct_hex).unwrap();

        let ct = c_vec_to_array(ctvec);
        let sk = sk_vec_to_array(skvec);

        let (ss, ok) = decapsulate(ct, sk);
        let shex = hex::encode(ss);
        println!("{}", shex);

    }

    #[test]
    #[ignore]
    fn known_answer_tests(){
        #[derive(Deserialize)]
        struct KAT {
            c: String,
            k: String,
            pk: String,
            sk: String
        }
        
        let f = File::open("src/tests/kat.json").expect("kat.json not found");
        let kats: Vec<KAT> = serde_json::from_reader(f).expect("Error reading kat.json");
        let mut i = 0;
        for kat in kats.into_iter(){
            i += 1;
            let katc = c_vec_to_array(hex::decode(kat.c).unwrap());
            let katsk = sk_vec_to_array(hex::decode(kat.sk).unwrap());
            let (k, _) = decapsulate(katc, katsk);
            assert_eq!(k , k_vec_to_array(hex::decode(kat.k).unwrap()));
            if i == 2{
                break
            }
        }
    
    }

    #[test]
    fn encap_decap() {
        let (pk, sk) = generate_key();
        let (c, k) = encapsulate(pk);
        let (result, _) = decapsulate(c, sk);
        assert_eq!(result, k);
    }

    fn c_vec_to_array(v: Vec<u8>)-> [u8; 1047]{
        let mut arr = [0u8; 1047];
        arr.copy_from_slice(&v[..]);
        arr
    }

    fn sk_vec_to_array(v: Vec<u8>)-> [u8;1600]{
        let mut arr = [0u8; 1600];
        arr.copy_from_slice(&v[..]);
        arr
    }

    fn k_vec_to_array(v: Vec<u8>)-> [u8;32]{
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&v[..]);
        arr
    }

}
