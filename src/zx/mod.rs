pub mod encoding {
    pub fn encode(f: [i8; 761])-> [u8; 191]{
        let mut c = [0u8; 191];
        let mut j = 0;
        for i in 0..190{
            let mut c0 = f[j] + 1;
            c0 += (f[j+1] + 1) << 2;
            c0 += (f[j+2] + 1) << 4;
            c0 += (f[j+3] + 1) << 6;
            c[i] = c0 as u8;
            j += 4;
        }
        c[190] = (f[760] +1) as u8;
        c
    }

    pub fn decode(c: &[u8])-> [i8; 761]{
        let mut f = [0i8; 761];
        let mut j = 0;
        for i in 0..190{
            let mut c0 = c[i];
            f[j] = ((c0 & 3) as i8) - 1 ;
            c0 >>= 2;
            f[j+1] = ((c0 & 3) as i8) - 1;
            c0 >>= 2;
            f[j+2] = ((c0 & 3) as i8) - 1;
            c0 >>= 2;
            f[j+3] = ((c0 & 3) as i8) - 1;
            j += 4;
        }
        f[760] = ((c[190] & 3) as i8) -1;
        f
    }
}

pub mod random {
    use rand::{Rng, ThreadRng};

    pub fn random_i32(rng: &mut ThreadRng)-> i32{
        rng.gen::<i32>()
    }

    pub fn min_max(arr: &mut[i32], x: usize, y: usize){
        if arr[y] < arr[x]{
            arr.swap(x, y);
        }
    }

    pub fn sort (x: &mut[i32], n: usize){
        if n < 2 {
            return
        }
        let mut top = 1;
        while top < (n-top) {
            top += top;
        }
        let mut p = top;
        while p > 0 {
            for i in 0..(n-p){
                if i & p == 0 {
                    min_max(x, i, i+p)
                }
            }
            let mut q = top;    
            while q > p {
                for i in 0..(n-q){
                    if i & p == 0 {
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
}
