use rand::{Rng, ThreadRng};

pub fn random_i32(rng: &mut ThreadRng)-> i32{
    rng.gen::<i32>()
}

// minmax swaps x[0] and y[0] if y[0] < x[0]
// #[inline]
// pub fn min_max(x: &mut[i32], y: &mut[i32]){
//     if y[0] < x[0]{
//         let tmp_x = x[0];
//         x[0] = y[0];
//         y[0] = tmp_x;
//     }
// }

pub fn min_max(arr: &mut[i32], x: usize, y: usize){
    if arr[y] < arr[x]{
        arr.swap(x, y);
    }
}

pub fn sort (x: &mut[i32], n: usize){
    if n < 2 {
        return
    }
    //floor(log2 n)
    let mut top = 1;
    while top < n-top {
        top += top;
    }
    let mut p = top;
    while p > 0 {
        for i in 0..n-p{
            if i & p == 0 {
                //min_max(&mut x[i..], &mut x[i+p..])
                min_max(x, i, i+p)
            }
        }
        let mut q = top;    
        while q > p {
            for i in 0..n-q{
                if i & p == 0 {
                    //min_max(&mut x[(i+p)..], &mut x[(i+q)..])
                    min_max(x, i+p, i+q)
                }
            }
            q >>= 1;
        }
        p >>= 1;
    }
}

pub fn random_small(g: &mut [i8; 761], rng: &mut ThreadRng){
    for i in 0..761{
        let r = random_i32(&mut *rng);
        g[i] = ((((1_073_741_823 & (r as u32) ) * 3) >> 30) as i8) - 1;
    }
}

pub fn random_tsmall(f: &mut [i8; 761], rng: &mut ThreadRng){
    let mut r = [0i32; 761];
    for i in 0..761{
        let x = random_i32(&mut *rng);
        r[i] = x;
    }
    for i in 0..286{
        r[i] &= -2;
    }
    for i in 286..761{
        r[i] = (r[i] & -3) | 1
    }
    sort(&mut r, 761);
    for i in 0..761{
        f[i] = ((r[i] & 3) as i8) -1;
    }
}