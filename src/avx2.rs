#[cfg(any(target_arch = "x86", target_arch = "x86_64"),)
    //target_feature = "avx2")
]

use std::arch::x86_64::{
    /*_mm256_aesenc_epi128, _mm256_aesdec_epi128,*/_mm256_set_m128i, __m256i, __m256d, _mm256_set_epi64x, _mm256_set_pd,
    _mm256_extract_epi64, _mm256_sub_epi64, _mm256_div_pd,
    _mm256_mul_epi32, _mm256_add_epi64, _mm256_mulhi_epi16, _mm256_store_pd, _mm256_hadds_epi16,
    __m128i, /*_mm_set_epi64x,*/ _mm_aesdec_si128, _mm_aesenc_si128, /*_mm256_castsi256_pd,*/ _mm256_castsi256_si128,
    _mm256_setzero_si256, _mm256_setzero_pd, _mm_setzero_si128, _mm256_and_si256, _mm256_hsub_epi32, _mm256_sqrt_pd,
};

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub (crate) struct m256i(pub __m256i);

impl m256i {
    pub (crate) fn zero() -> m256i {
        unsafe {m256i(_mm256_setzero_si256())}
    }
    /*pub (crate) fn from_u8(bytes: &[u8]) -> m256i {
        let u0: u32 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let u1: u32 = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let u2: u32 = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let u3: u32 = u32::from_le_bytes(bytes[12..16].try_into().unwrap());

        unsafe {m256i(_mm256_set_epi64x(u0 as i64, u1 as i64, u2 as i64, u3 as i64))}
    }*/

    /*pub fn from_i64(a: i64, b: i64, c: i64, d: i64) -> m256i {
        unsafe {m256i(_mm256_set_epi64x(a, b, c, d))}
    }*/

    pub (crate) fn from_u128_one(un: u128) -> m256i {
        let bytes: [u8; 16] = un.to_le_bytes();
        let a: u32 = u32::from_le_bytes(flashbuf(&bytes[0..4]));
        let b: u32 = u32::from_le_bytes(flashbuf(&bytes[4..8]));
        let c: u32 = u32::from_le_bytes(flashbuf(&bytes[8..12]));
        let d: u32 = u32::from_le_bytes(flashbuf(&bytes[12..16]));
        unsafe {m256i(_mm256_set_epi64x(a as i64, b as i64, c as i64, d as i64))}
    }

    pub (crate) fn from_u128x2(un: [u128; 2]) -> m256i {
        let bytes_l: [u8; 16] = un[0].to_le_bytes();
        let bytes_r: [u8; 16] = un[1].to_le_bytes();
        let a: u64 = u64::from_le_bytes(flashbufx2(&bytes_l[0..8]));
        let b: u64 = u64::from_le_bytes(flashbufx2(&bytes_l[8..16]));
        let c: u64 = u64::from_le_bytes(flashbufx2(&bytes_r[0..8]));
        let d: u64 = u64::from_le_bytes(flashbufx2(&bytes_r[8..16]));
        unsafe {m256i(_mm256_set_epi64x(a as i64, b as i64, c as i64, d as i64))}
    }

    /*pub fn aesdec(&self, key: m256i) -> m256i {
        unsafe { m256i(_mm256_aesdec_epi128(self.0, key.0)) }
    }
    pub fn aesenc(&self, key: m256i) -> m256i {
        unsafe { m256i(_mm256_aesenc_epi128(self.0, key.0)) }
    }*/
    pub fn as_u128(&self) -> u128 {
        unsafe {
            let i1: u128 = _mm256_extract_epi64(self.0, 0) as u128;
            let i2: u128 = _mm256_extract_epi64(self.0, 1) as u128;
            let i3: u128 = _mm256_extract_epi64(self.0, 2) as u128;
            let i4: u128 = _mm256_extract_epi64(self.0, 3) as u128;
            /*let mut u: u128 = 0u128;
            u += i1 as u128;
            u += i2 as u128;
            u += i3 as u128;
            u += i4 as u128;*/
            //i1 + i2 + i3 + i4
            i1.saturating_add(i2).saturating_add(i3).saturating_add(i4)
        }
    }

    pub fn as_u128x2(&self) -> [u128; 2] {
        unsafe {
            let i1: u128 = _mm256_extract_epi64(self.0, 0) as u128;
            let i2: u128 = _mm256_extract_epi64(self.0, 1) as u128;
            let i3: u128 = _mm256_extract_epi64(self.0, 2) as u128;
            let i4: u128 = _mm256_extract_epi64(self.0, 3) as u128;
            /*let mut u: u128 = 0u128;
            u += i1 as u128;
            u += i2 as u128;
            u += i3 as u128;
            u += i4 as u128;*/
            [i1.saturating_add(i2), i3.saturating_add(i4)]
        }
    }

    pub fn add(a: m256i, b: m256i) -> m256i {
        unsafe{m256i(_mm256_add_epi64(a.0, b.0))}
    }

    pub fn sub(a: m256i, b: m256i) -> m256i {
        unsafe{m256i(_mm256_sub_epi64(a.0, b.0))}
    }

    pub fn mul(a: m256i, b: m256i) -> m256i {
        unsafe{m256i(_mm256_mul_epi32(a.0, b.0))}
    }

    pub fn and(a: m256i, b: m256i) -> m256i {
        unsafe{m256i(_mm256_and_si256(a.0, b.0))}
    }

    /*pub fn as_m256d(&self) -> m256d {
        unsafe{m256d(_mm256_castsi256_pd(self.0))}
    }*/

    pub fn palindrome_hell(a: m256i, b: m256i) -> m256i {
        let mulhi: m256i = unsafe{m256i(_mm256_mulhi_epi16(a.0, b.0))};
        let mybad: m256i = unsafe{m256i(_mm256_hadds_epi16(a.0, b.0))};
        //unsafe{m256i(_mm256_mulhi_epi16(mulhi.0, mybad.0))}
        //m256i::from_m128i(mulhi.as_m128i().aesenc(mybad.as_m128i()), mybad.as_m128i().aesenc(mulhi.as_m128i()))
        unsafe{m256i(_mm256_hsub_epi32(mulhi.0, mybad.0))}
    }

    pub fn from_m128i(m1: m128i, m2: m128i) -> m256i {
        unsafe {m256i(_mm256_set_m128i(m1.0, m2.0))}
    }

    pub fn as_m128i(&self) -> m128i {
        unsafe{m128i(_mm256_castsi256_si128(self.0))}
    }
}

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub (crate) struct m256d(pub __m256d);

impl m256d {
    pub fn zero() -> m256d {
        unsafe {m256d(_mm256_setzero_pd())}
    }
    
    /*pub fn from_f64(a: f64, b: f64, c: f64, d: f64) -> m256d {
        unsafe {m256d(_mm256_set_pd(a, b, c, d))}
    }*/

    pub fn div(a: m256d, b: m256d) -> m256d {
        unsafe {m256d(_mm256_div_pd(a.0, b.0))}
    }

    pub fn sqrt(&mut self) {
        unsafe {*self = m256d(_mm256_sqrt_pd(self.0))}
    }

    pub fn from_u128_one(un: u128) -> m256d {
        let bytes: [u8; 16] = un.to_le_bytes();
        let a: u32 = u32::from_le_bytes(flashbuf(&bytes[0..4]));
        let b: u32 = u32::from_le_bytes(flashbuf(&bytes[4..8]));
        let c: u32 = u32::from_le_bytes(flashbuf(&bytes[8..12]));
        let d: u32 = u32::from_le_bytes(flashbuf(&bytes[12..16]));
        unsafe {m256d(_mm256_set_pd(a as f64, b as f64, c as f64, d as f64))}
    }

    pub (crate) fn from_u128x2(un: [u128; 2]) -> m256d {
        let bytes_l: [u8; 16] = un[0].to_le_bytes();
        let bytes_r: [u8; 16] = un[1].to_le_bytes();
        let a: u64 = u64::from_le_bytes(flashbufx2(&bytes_l[0..8]));
        let b: u64 = u64::from_le_bytes(flashbufx2(&bytes_l[8..16]));
        let c: u64 = u64::from_le_bytes(flashbufx2(&bytes_r[0..8]));
        let d: u64 = u64::from_le_bytes(flashbufx2(&bytes_r[8..16]));
        unsafe {m256d(_mm256_set_pd(a as f64, b as f64, c as f64, d as f64))}
    }

    /*pub fn as_u128(&self) -> u128 {
        unsafe {
            let mut f1: f64 = 0.0;
            let mut f2: f64 = 0.0;
            let f1_ptr: *mut f64 = &mut f1;
            let f2_ptr: *mut f64 = &mut f2;
            _mm256_store_pd(f1_ptr, self.0);
            _mm256_store_pd(f2_ptr, self.0);
            f1 as u128 + f2 as u128
        }
    }*/

    pub fn as_u128x2(&self) -> [u128; 2] {
        unsafe {
            let mut f1: f64 = 0.0;
            let mut f2: f64 = 0.0;
            let mut f3: f64 = 0.0;
            let mut f4: f64 = 0.0;
            _mm256_store_pd(&mut f1 as *mut f64, self.0);
            _mm256_store_pd(&mut f2 as *mut f64, self.0);
            _mm256_store_pd(&mut f3 as *mut f64, self.0);
            _mm256_store_pd(&mut f4 as *mut f64, self.0);
            [f1 as u128 + f2 as u128, f3 as u128 + f4 as u128]
        }
    }

    /*pub fn from_m256i(m: m256i) -> m256d {
        unsafe {m256d(_mm256_castsi256_pd(m.0))}
    }*/
}

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m128i(pub __m128i);

impl m128i {
    pub fn zero() -> m128i {
        unsafe {m128i(_mm_setzero_si128())}
    }
    /*pub fn from_u128(un: u128) -> m128i {
        let bytes: [u8; 16] = un.to_le_bytes();
        let a: u64 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let b: u64 = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        unsafe {m128i(_mm_set_epi64x(a as i64, b as i64))}
    }*/
    pub fn aesdec(&self, key: m128i) -> m128i {
        unsafe { m128i(_mm_aesdec_si128(self.0, key.0)) }
    }
    pub fn aesenc(&self, key: m128i) -> m128i {
        unsafe { m128i(_mm_aesenc_si128(self.0, key.0)) }
    }
}

const fn flashbuf(wut: &[u8]) -> [u8; 4] {
    [wut[0], wut[1], wut[2], wut[3]]
}

const fn flashbufx2(wut: &[u8]) -> [u8; 8] {
    [wut[0], wut[1], wut[2], wut[3], wut[4], wut[5], wut[6], wut[7]]
    //unsafe{*(wut.as_ptr() as *mut [u8; 8])}
}
