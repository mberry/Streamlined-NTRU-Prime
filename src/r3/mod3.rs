#![allow(dead_code)]


pub fn freeze(a: i32)-> i8{
    let b = a - (3 * ((10923 * a) >> 15));
    let c = b - (3 * ((89_478_485 * b + 134_217_728) >> 28));
    c as i8
}

pub fn product (a : i8, b :i8) -> i8{
    a * b
}

pub fn reciprocal(a: i8)-> i8{
    a
}

pub fn quotient(a: i8, b:i8)-> i8{
    product(a, reciprocal(b))
}

pub  fn minus_product(a: i8, b: i8, c: i8)->i8{
    freeze(a as i32 - b as i32 * c as i32)
}

pub fn plus_product(a: i8, b: i8, c: i8)->i8{
    freeze(a as i32 + b as i32 * c as i32)
}

pub fn sum (a: i8, b: i8)-> i8{
    freeze(a as i32 + b as i32)
}

pub fn mask_set(x: i8)-> isize{
    (-x * x) as isize
}

