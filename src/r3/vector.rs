use r3::mod3;

pub fn swap(x: &mut[i8], y: &mut[i8], bytes: usize, mask: isize){
    let c = mask as i8;
    for i in 0..bytes{
        let t = c & (x[i] ^ y[i]);
        x[i] ^= t;
        y[i] ^= t;
    }
}

pub fn product(z: &mut [i8], n: usize, x: &[i8], c :i8){
    for i in 0..n{
        z[i] = mod3::product(x[i], c);
    }
}

pub fn minus_product(z: &mut[i8], n: usize, y: &[i8], c :i8){
    for i in 0..n{
        let x = z[i];
        z[i] = mod3::minus_product(x, y[i], c);
    }
}

pub fn shift(z: &mut[i8], n :usize){
    for i in (1..n).rev(){
        z[i] = z[i-1];
    }
    z[0] = 0;
}
