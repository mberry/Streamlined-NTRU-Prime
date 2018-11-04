use rand;
use rand::Rng;

pub fn random_i32()-> i32{
    let mut rng = rand::thread_rng();
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

