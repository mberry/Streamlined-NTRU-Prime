extern crate rand;

use rand;


pub fn random_i32()-> i32{
    let mut rng = rand::thread_rng();
    rng.gen::<i32>();
}

// minmax swaps x[0] and y[0] if y[0] < x[0]
#[inline]
pub fn min_max(x: &mut[i32], y: &mut[i32]){
    if y[0] < x[0]{
        let temp_x = x[0];
        x[0] = y[0];
        y[0] = temp_x;
    }
}

//messy
pub fn sort (x: &mut[i32], n isize){
    if n < 2 {
        break;
    }
    //floor(log2 n)
    let top = 1;
    for top < n-top {
        top += top;
    }
    let mut p = top;
    while p > 0 {
        for i in 0..n-p{
            if i & p == 0 {
                min_max(x[i..], x[i+p..])
            }
        }
        let mut q = top;    
        while q > p {
            for i in 0..n-q{
                if i & p == 0 {
                    min_max(x[(i+p)..], x[(i+q)..])
                }
            }
            q >>= 1;
        }
        p >>= 1;
    }
}