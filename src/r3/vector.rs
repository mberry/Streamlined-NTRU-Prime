#![allow(dead_code)]
use mod3;


// Swap swaps x and y if mask is -1. If mask is 0, x and y retain
// their original values.
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
    for i in (0..n-1).rev(){
        z[i] = z[i-1];
    }
    z[0] = 0;
}
