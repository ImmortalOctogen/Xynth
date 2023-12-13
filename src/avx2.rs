use std::arch::x86_64::_mm256_set_m128i;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"),)
    //target_feature = "avx2")
]

use std::arch::x86_64::{
    _mm256_aesenc_epi128, _mm256_aesdec_epi128, __m256i, __m256d, _mm256_set_epi64x, _mm256_set_pd,
    _mm256_extract_epi64, _mm256_sub_epi64, _mm256_div_pd,
    _mm256_mul_epi32, _mm256_add_epi64, _mm256_mulhi_epi16, _mm256_store_pd, _mm256_hadds_epi16,
    __m128i, _mm_set_epi64x, _mm_aesdec_si128, _mm_aesenc_si128,
};

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m256i(pub __m256i);

impl m256i {
    /*pub fn zero() -> m256i {
        unsafe {m256i(_mm256_setzero_si256())}
    }*/
    pub fn from_u8(bytes: &[u8]) -> m256i {
        //assert_eq!(bytes.len(), 48); //set to debug_assert_eq!
        //println!("passing {}", bytes.len());

        let u0: u32 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let u1: u32 = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let u2: u32 = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let u3: u32 = u32::from_le_bytes(bytes[12..16].try_into().unwrap());

        unsafe {m256i(_mm256_set_epi64x(u0 as i64, u1 as i64, u2 as i64, u3 as i64))}
    }

    /*pub fn from_i64(a: i64, b: i64, c: i64, d: i64) -> m256i {
        unsafe {m256i(_mm256_set_epi64x(a, b, c, d))}
    }*/

    pub fn from_u128_one(un: u128) -> m256i {
        let bytes: [u8; 16] = un.to_le_bytes();
        let a: u32 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let b: u32 = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let c: u32 = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let d: u32 = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        unsafe {m256i(_mm256_set_epi64x(a as i64, b as i64, c as i64, d as i64))}
    }

    pub fn aesdec(&self, key: m256i) -> m256i {
        unsafe { m256i(_mm256_aesdec_epi128(self.0, key.0)) }
    }
    pub fn aesenc(&self, key: m256i) -> m256i {
        unsafe { m256i(_mm256_aesenc_epi128(self.0, key.0)) }
    }
    pub fn as_u128(&self) -> u128 {
        unsafe {
            let i1: i64 = _mm256_extract_epi64(self.0, 0);
            let i2: i64 = _mm256_extract_epi64(self.0, 1);
            let i3: i64 = _mm256_extract_epi64(self.0, 2);
            let i4: i64 = _mm256_extract_epi64(self.0, 3);
            let mut u: u128 = 0u128;
            u += i1 as u128;
            u += i2 as u128;
            u += i3 as u128;
            u += i4 as u128;
            u
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

    /*pub fn as_m256d(&self) -> m256d {
        unsafe{m256d(_mm256_castsi256_pd(self.0))}
    }*/

    pub fn palindrome_hell(a: m256i, b: m256i) -> m256i {
        //println!("1");
        let mulhi: m256i = unsafe{m256i(_mm256_mulhi_epi16(a.0, b.0))};
        //println!("2");
        let mybad: m256i = unsafe{m256i(_mm256_hadds_epi16(a.0, b.0))};
        //println!("3");
        unsafe{m256i(_mm256_mulhi_epi16(mulhi.0, mybad.0))}
    }

    pub fn from_m128i(m1: m128i, m2: m128i) -> m256i {
        unsafe {m256i(_mm256_set_m128i(m1.0, m2.0))}
    }
}

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m256d(pub __m256d);

impl m256d {
    /*pub fn zero() -> m256d {
        unsafe {m256d(_mm256_setzero_pd())}
    }*/
    
    /*pub fn from_f64(a: f64, b: f64, c: f64, d: f64) -> m256d {
        unsafe {m256d(_mm256_set_pd(a, b, c, d))}
    }*/

    pub fn div(a: m256d, b: m256d) -> m256d {
        unsafe {m256d(_mm256_div_pd(a.0, b.0))}
    }

    pub fn from_u128_one(un: u128) -> m256d {
        let bytes: [u8; 16] = un.to_le_bytes();
        let a: u32 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let b: u32 = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let c: u32 = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let d: u32 = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        unsafe {m256d(_mm256_set_pd(a as f64, b as f64, c as f64, d as f64))}
    }

    pub fn as_u128(&self) -> u128 {
        unsafe {
            let mut u: u128 = 0u128;
            let mut f1: f64 = 0.0;
            let mut f2: f64 = 0.0;
            let f1_ptr: *mut f64 = &mut f1;
            let f2_ptr: *mut f64 = &mut f2;
            _mm256_store_pd(f1_ptr, self.0);
            _mm256_store_pd(f2_ptr, self.0);
            u += f1 as u128;
            u += f2 as u128;
            u
        }
    }
}

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m128i(pub __m128i);

impl m128i {
    pub fn from_u128(un: u128) -> m128i {
        let bytes: [u8; 16] = un.to_le_bytes();
        let a: u64 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let b: u64 = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        unsafe {m128i(_mm_set_epi64x(a as i64, b as i64))}
    }
    pub fn aesdec(&self, key: m128i) -> m128i {
        unsafe { m128i(_mm_aesdec_si128(self.0, key.0)) }
    }
    pub fn aesenc(&self, key: m128i) -> m128i {
        unsafe { m128i(_mm_aesenc_si128(self.0, key.0)) }
    }
}