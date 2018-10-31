use modq;

pub fn swap(x: &mut[i16], y: &mut[i16], bytes: usize, mask isize){
    let c = mask as i16;
    for i in 0..bytes{
        let t = c & (x[i] ^ y[i]);
        x[i] ^= t;
        y[i] ^= t;
    }
}

pub fn product(z: &mut [i16], n: usize, x: &[i16], c :i16){
    for i in 0..n{
        z[i] = modq::product(x[i], c);
    }
}

pub fn minus_product(z: &mut[i16], n: usize, y: &[i16], c :i16){
    for i in 0..n{
        let x = z[i];
        z[i] = modq::minus_product(x, y[i], c);
    }
}

pub fn shift(z: &mut[i16], n :usize){
    for i in (0..n-1).rev(){
        z[i] = z[i-1];
    }
    z[0] = 0;
}