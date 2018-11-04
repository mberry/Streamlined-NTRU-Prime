
pub fn encode(f: [i8; 761])-> [u8; 191]{
    let c = [0u8; 191];
    let mut j = 0;
    for i in 0..190{
        let c0 = f[j] + 1;
        c0 += (f[j+1] + 1) << 2;
        c0 += (f[j+2] + 1) << 4;
        c0 += (f[j+3] + 1) << 6;
        c[i] = c0 as u8;
        j += 4;
    }
    c[190] = (f[760] +1) as u8;
    c
}

fn decode(c: &[u8])-> [i8; 761]{
    let f = [0i8; 761];

}
