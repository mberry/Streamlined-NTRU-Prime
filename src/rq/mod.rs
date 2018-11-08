pub mod encoding;
pub mod modq;
pub mod vector;

use std::num::Wrapping;

fn swap_int(x : isize, y : isize, mask: isize)-> (isize, isize){
    let t = mask & (x ^ y);
    (x ^ t, y ^ t)
}

fn smaller_mask(x: isize, y: isize) -> isize{
    (x-y) >> 31
}

pub fn reciprocal3(r: &mut [i16; 761], s: [i8; 761])-> isize{
    const LOOPS: usize = 2*761 + 1;

    let mut f = [0i16; 761 + 1];
    f[0] = -1;
    f[1] = -1;
    f[761] = 1;
    let mut g = [0i16; 761 + 1];
    for i in 0..761{
        g[i] = (3 * s[i]) as i16;
    }
    let mut d = 761;
    let mut e = 761;
    let mut u = [0i16; LOOPS +1]; 
    let mut v = [0i16; LOOPS +1];
    v[0] = 1;
    
    for _ in 0..LOOPS{
        let c = modq::quotient(g[761], f[761]);
        vector::minus_product(&mut g, 761 + 1, &f, c);
        vector::shift(&mut g, 761 + 1);
        vector::minus_product(&mut v, LOOPS+1, &u, c);
        vector::shift(&mut v, LOOPS+1);
        e -= 1;
        let m = smaller_mask(e, d) & modq::mask_set(g[761]);
        let (e, d) = swap_int(e, d, m);
        vector::swap(&mut f, &mut g, 761+1, m);
        vector::swap(&mut u, &mut v, LOOPS+1, m);
    } 
    vector::product(r, 761, &u[761..], modq::reciprocal(f[761]));
    smaller_mask(0, d)
}

pub fn round3(h: &mut[i16; 761]){
    let f: [i16; 761] = *h;
    for i in 0..761{
        let inner = 21846i32 * (f[i]+2295) as i32;
        h[i] = (((inner +32768)>>16)*3 - 2295) as i16;
    }
}

pub fn mult(h: &mut [i16; 761], f: [i16; 761], g: [i8; 761]){
    let mut fg = [0i16; 761*2 -1];
    for i in 0..761{
        let mut r = 0i16;
        for j in 0..=i {
            r = modq::plus_product(r, f[j], g[i-j] as i16);
        }
        fg[i] = r;
    }
    for i in 761..(761*2-1){
        let mut r = 0i16;
        for j in (i-761+1)..761{
            r = modq::plus_product(r, f[j], g[i-j] as i16)
        }
        fg[i] = r;
    }
    for i in (761..(761*2)-1).rev(){
        fg[i-761] = modq::sum(fg[i-761], fg[i]);
        fg[i-761+1] = modq::sum(fg[i-761+1], fg[i]);
    }
    h[..761].clone_from_slice(&fg[..761]);
}

