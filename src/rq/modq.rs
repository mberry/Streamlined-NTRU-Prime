pub fn freeze(a : i32)-> i16 {
	let mut b = a;
	b -= 4_591 * ((228 * b) >> 20);
	b -= 4_591 * ((58_470*b + 134_217_728) >> 28);
	b as i16
}

pub fn product(a: i16, b: i16)-> i16{
    freeze(a as i32 * b as i32)
}

pub fn square(a: i16)-> i16{
    let a32 = a as i32;
    freeze(a32 * a32)
}

pub fn reciprocal(a1: i16)-> i16 {
	let a2 = square(a1);
	let a3 = product(a2, a1);
	let a4 = square(a2);
	let a8 = square(a4);
	let a16 = square(a8);
	let a32 = square(a16);
	let a35 = product(a32, a3);
	let a70 = square(a35);
	let a140 = square(a70);
	let a143 = product(a140, a3);
	let a286 = square(a143);
	let a572 = square(a286);
	let a1144 = square(a572);
	let a1147 = product(a1144, a3);
	let a2294 = square(a1147);
	let a4588 = square(a2294);
	product(a4588, a1)
}

pub fn quotient(a: i16, b: i16)-> i16{
    product(a, reciprocal(b))
}

pub fn minus_product(a: i16, b: i16, c: i16)-> i16{
freeze(a as i32 - (b as i32 * c as i32))
}

pub fn plus_product(a: i16, b: i16, c:i16)-> i16{
    freeze(a as i32 + (b as i32 * c as i32))
}

pub fn sum(a: i16, b: i16)-> i16{
    freeze(a as i32 + b as i32)
}

pub fn mask_set(x: i16)-> isize{
    let mut r = (x as u16) as i32;
    r = -r;
    r >>= 30;
    r as isize
}

