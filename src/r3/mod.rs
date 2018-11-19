pub mod mod3;
mod vector;

fn swap_int(x : isize, y : isize, mask: isize)-> (isize, isize){
    let t = mask & (x ^ y);
    (x ^ t, y ^ t)
}

fn smaller_mask(x: isize, y: isize) -> isize{
    (x-y) >> 31
}

pub fn reciprocal(s: [i8; 761])-> (isize, [i8; 761]){
    const LOOPS: usize = 2*761 + 1;
    let mut r = [0i8; 761];
    let mut f = [0i8; 761 + 1];
    f[0] = -1;
    f[1] = -1;
    f[761] = 1;

    let mut g = [0i8; 761 + 1];
    g[..761].clone_from_slice(&s[..761]);
    let mut d = 761;
    let mut e = 761;
    let mut u = [0i8; LOOPS + 1];
    let mut v = [0i8; LOOPS + 1];
    v[0] = 1;
    
    for _ in 0..LOOPS{
        let c = mod3::quotient(g[761], f[761]);
        vector::minus_product(&mut g, 761 + 1, &f, c);
        vector::shift(&mut g, 761 + 1);
        vector::minus_product(&mut v, LOOPS+1, &u, c);
        vector::shift(&mut v, LOOPS+1);
        e -= 1;
        let m = smaller_mask(e, d) & mod3::mask_set(g[761]);
        let (e_tmp, d_tmp) = swap_int(e, d, m);
        e = e_tmp;
        d = d_tmp;
        vector::swap(&mut f, &mut g, 761+1, m);
        vector::swap(&mut u, &mut v, LOOPS+1, m);
    } 

    vector::product(&mut r, 761, &u[761..], mod3::reciprocal(f[761]));
    (smaller_mask(0, d), r)
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
