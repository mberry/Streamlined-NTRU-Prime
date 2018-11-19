#![allow(dead_code)]
pub mod mod3;
pub mod vector;

// swapInt swaps x and y if mask is -1. If mask is 0, x and y retain
// their original values.
fn swap_int(x : isize, y : isize, mask: isize)-> (isize, isize){
    let t = mask & (x ^ y);
    (x ^ t, y ^ t)
}

// smallerMask compares x and y, returning -1 if y > x, and 0 otherwise
fn smaller_mask(x: isize, y: isize) -> isize{
    (x-y) >> 31
}

pub fn reciprocal(r: &mut [i8; 761], s: [i8; 761])-> isize{
    // f starts as the modulus of R3
    let mut f = [0i8; 761 + 1];
    f[0] = -1;
    f[1] = -1;
    f[761] = 1;

    // g starts as s
    let mut g = [0i8; 761 + 1];
    g[..761].clone_from_slice(&s[..761]);
    let mut d = 761;
    let mut e = 761;
    let loops = 2*761 + 1;
    let mut u = [0i8; 2*761+2]; // loops + 1
    let mut v = [0i8; 2*761+2];
    v[0] = 1;
    
    for _ in 0..loops{
        // c = (lc(g)/lc(f)) % 3
        let c = mod3::quotient(g[761], f[761]);
        // g = g - f*c; g <<= 1
        vector::minus_product(&mut g, 761 + 1, &f, c);
        vector::shift(&mut g, 761 + 1);
        // v = v - u*c
        vector::minus_product(&mut v, loops+1, &u, c);
        vector::shift(&mut v, loops+1);
        // swap (e,d), (f,g), and (u,v) if d > e and lc(g) != 0
        e -= 1;
        let m = smaller_mask(e, d) & mod3::mask_set(g[761]);
        let (e_tmp, d_tmp) = swap_int(e, d, m);
        e = e_tmp;
        d = d_tmp;
        vector::swap(&mut f, &mut g, 761+1, m);
        vector::swap(&mut u, &mut v, loops+1, m);
    } 

    vector::product(&mut r, 761, &u[761..], mod3::reciprocal(f[761]));
    smaller_mask(0, d)
}

pub fn mult(h: &mut [i8; 761], f: [i8; 761], g: [i8; 761]){
    let mut fg = [0i8; 761*2 -1];
    for i in 0..761{
        let mut r = 0i8;
        for j in 0..=i {
            r = mod3::plus_product(r, f[j], g[i-j]);
        }
        fg[i] = r;
    }
    for i in 761..(761*2-1){
        let mut r = 0i8;
        for j in (i-761+1)..761{
            r = mod3::plus_product(r, f[j], g[i-j])
        }
        fg[i] = r;
    }
    for i in (761..(761*2)-1).rev(){
        let tmp1 = mod3::sum(fg[i-761], fg[i]);
        fg[i-761] = tmp1;
        let tmp2 =mod3::sum(fg[i-761+1], fg[i]);
        fg[i-761+1] = tmp2;
    }

    h[..761].clone_from_slice(&fg[..761]);
}
