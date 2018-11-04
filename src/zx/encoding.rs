
pub fn encode(f: [i8; 761])-> [u8; 191]{
    let mut c = [0u8; 191];
    let mut j = 0;
    for i in 0..190{
        let mut c0 = f[j] + 1;
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
    let mut f = [0i8; 761];
    let mut j = 0;
    for i in 0..190{
        let mut c0 = c[i];
        f[j] = ((c0 & 3) - 1) as i8;
        c0 >>= 2;
        f[j+1] = ((c0 & 3) - 1) as i8;
        c0 >>= 2;
        f[j+2] = ((c0 & 3) - 1) as i8;
        c0 >>= 2;
        f[j+3] = ((c0 & 3) - 1) as i8;
        j += 4;
    }
    f[760] = ((c[190] & 3) -1 ) as i8;
    f
}
