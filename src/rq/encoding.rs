use rQ::modQ;

pub fn encode(f: [i16; 761]) -> [u8; 1218]{
    const QSHIFT: i32 = 2295;
    let mut f0 = 0i32;
    let mut f1 = 0i32;
    let mut f2 = 0i32;
    let mut f3 = 0i32;
    let mut f4 = 0i32;

    let mut c = [0u8; 1218];

    let mut j = 0;
    let mut k = 0;
    for i in 0..152{
        f0 = (f[j+0] as i32 + QSHIFT) * 1;
        f1 = (f[j+1] as i32 + QSHIFT) * 3;
        f2 = (f[j+2] as i32 + QSHIFT) * 9;
        f3 = (f[j+3] as i32 + QSHIFT) * 27;
        f4 = (f[j+4] as i32 + QSHIFT) * 81;

        j += 5;
        f0 += f1 << 11;
        c[k+0] = f0 as u8;
        f0 >>= 8;
        c[k+1] = f0 as u8;
        f0 >>= 8;
        f0 += f2 << 6;
        c[k+2] = f0 as u8;
        f0 >>= 8;
        c[k+3] = f0 as u8;
        f0 >>= 8;
        f0 += f3 << 1;
        c[k+4] = f0 as u8;
        f0 >>= 8;
        f0 += f4 << 4;
        c[k+5] = f0 as u8;
        f0 >>= 8;
        c[k+6] = f0 as u8;
        f0 >>= 8;
        c[k+7] = f0 as u8;
        k += 8;
    }

    f0 = f[760] as i32 + QSHIFT;
    c[1216] = f0 as u8;
    c[1217] = (f0 >> 8) as u8;
    c
}

pub fn decode(c : &[u8])-> [i16; 761]{

    const QSHIFT: u32 = 2295;
    const Q: u32 = 4591;

    let mut f0 = 0u32;
    let mut f1 = 0u32;
    let mut f2 = 0u32;
    let mut f3 = 0u32;
    let mut f4 = 0u32;

    let mut c0 = 0u32;
    let mut c1 = 0u32;
    let mut c2 = 0u32;
    let mut c3 = 0u32;
    let mut c4 = 0u32;
    let mut c5 = 0u32;
    let mut c6 = 0u32;
    let mut c7 = 0u32;

    let mut f = [0i16; 761];

    let mut j = 0;
    let mut k = 0;

    for i in 0..152{
        c0 = c[j] as u32;
        c1 = c[j+1] as u32;
        c2 = c[j+2] as u32;
        c3 = c[j+3] as u32;
        c4 = c[j+4] as u32;
        c5 = c[j+5] as u32;
        c6 = c[j+6] as u32;
        c7 = c[j+7] as u32;

        j += 8;
        c6 += c7 << 8;
        f4 = (103_564*c6 + 405*(c5+1)) >> 19;
		c5 += c6 << 8;
		c5 -= (f4 * 81) << 4;
		c4 += c5 << 8;
		f3 = (9_709 * (c4 + 2)) >> 19;
		c4 -= (f3 * 27) << 1;
		c3 += c4 << 8;
		f2 = (233_017*c3 + 910*(c2+2)) >> 19;
		c2 += c3 << 8;
		c2 -= (f2 * 9) << 6;
		c1 += c2 << 8;
		f1 = (21_845*(c1+2) + 85*c0) >> 19;
		c1 -= (f1 * 3) << 3;
		c0 += c1 << 8;
        f0 = c0;

        f[k] = modQ::freeze((f0 + Q - QSHIFT) as i32);
        f[k+1] = modQ::freeze((f1 + Q - QSHIFT) as i32);
        f[k+2] = modQ::freeze((f2 + Q - QSHIFT) as i32);
        f[k+3] = modQ::freeze((f3 + Q - QSHIFT) as i32);
        f[k+4] = modQ::freeze((f4 + Q - QSHIFT) as i32);
        k += 5;
    }

    c0 = c[1216] as u32;
    c1 = c[1217] as u32;
    c0 += c1 << 8;
    f[760] = modQ::freeze((c0 + Q - QSHIFT) as i32);
    f
}

pub fn encode_rounded(f: [i16; 761])-> [u8; 1015]{
    let (mut f0, mut f1, mut f2) = (0i32, 0i32, 0i32);
    const QSHIFT: i32 = 2295;

    let mut c = [0u8; 1015];

    let mut j = 0;
    let mut k = 0;

    for i in 0..253{
        f0 = f[j+0] as i32 + QSHIFT;
        f1 = f[j+1] as i32 + QSHIFT;
        f2 = f[j+2] as i32 + QSHIFT;
        j += 3;
		f0 = (21846 * f0) >> 16;
		f1 = (21846 * f1) >> 16;
		f2 = (21846 * f2) >> 16;
		f2 *= 3;
		f1 += f2 << 9;
		f1 *= 3;
        f0 += f1 << 9;

		c[k+0] = f0 as u8;
		f0 >>= 8;
		c[k+1] = f0 as u8;
		f0 >>= 8;
		c[k+2] = f0 as u8;
		f0 >>= 8;
		c[k+3] = f0 as u8;
        k += 4
    }

    f0 = f[759] as i32 + QSHIFT;
	f1 = f[760] as i32 + QSHIFT;
	f0 = (21846 * f0) >> 16;
	f1 = (21846 * f1) >> 16;
	f1 *= 3;
	f0 += f1 << 9;

	c[1012] = f0 as u8;
	f0 >>= 8;
	c[1013] = f0 as u8;
	f0 >>= 8;
	c[1014] = f0 as u8;
    c
}

pub fn decode_rounded(c: &[u8])-> [i16; 761] {
	const Q: u32 = 4591;
	const QSHIFT: u32 = 2295;
    let (mut c0, mut c1, mut c2, mut c3) = (0u32, 0u32, 0u32, 0u32);
    let (mut f0, mut f1, mut f2) = (0u32, 0u32, 0u32);

    let mut f = [0i16; 761];
    let mut j = 0;
    let mut k = 0;

	for i in 0..253 {
		c0 = c[j+0] as u32;
		c1 = c[j+1] as u32;
		c2 = c[j+2] as u32;
		c3 = c[j+3] as u32;
		j += 4;

		f2 = (14_913_081*c3 + 58_254*c2 + 228*(c1+2)) >> 21;
		c2 += c3 << 8;
		c2 -= (f2 * 9) << 2;
		f1 = (89_478_485*c2 + 349_525*c1 + 1_365*(c0+1)) >> 21;
		c1 += c2 << 8;
		c1 -= (f1 * 3) << 1;
		c0 += c1 << 8;
		f0 = c0;

		f[k+0] = modQ.Freeze((f0*3 + Q - QSHIFT) as i32);
		f[k+1] = modQ.Freeze((f1*3 + Q - QSHIFT) as i32);
		f[k+2] = modQ.Freeze((f2*3 + Q - QSHIFT) as i32);
		k += 3;
	}

	c0 = c[1012] as u32;
	c1 = c[1013] as u32;
	c2 = c[1014] as u32;

	f1 = (89_478_485*c2 + 349_525*c1 + 1_365*(c0+1)) >> 21;
	c1 += c2 << 8;
	c1 -= (f1 * 3) << 1;
	c0 += c1 << 8;
	f0 = c0;

	f[759] = modQ.Freeze((f0*3 + Q - QSHIFT) as i32);
	f[760] = modQ.Freeze((f1*3 + Q - QSHIFT) as i32);

	f
}