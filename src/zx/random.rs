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