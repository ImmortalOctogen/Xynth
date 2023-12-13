use crate::avx2::*;

#[derive(PartialEq)]
pub enum OpCode {
    ENC = 0b111,
    DEC = 0b000,
    ADD = 0b001,
    SUB = 0b010,
    DIV = 0b011,
    MUL = 0b100,
    PAL = 0b101,
}

pub fn enc(inst: u128) -> m256i {
    let state: m128i = m128i::from_u128(inst);
    let halfr: u128 = (inst << 64) >> 64;
    let halfl: u128 = (inst >> 64) << 64;
    let state1: m128i = m128i::from_u128(halfr ^ halfl);
    let seed: u128 = halfr & halfl;
    //println!("passingENC");
    let m1: m128i = state.aesenc(m128i::from_u128(seed));
    let m2: m128i = state1.aesenc(m128i::from_u128(seed));
    m256i::from_m128i(m1, m2)
}

pub fn dec(inst: u128) -> m256i {
    /*let state: m256i = m256i::from_u128_one(inst);
    let halfr: u128 = (inst << 64) >> 64;
    let halfl: u128 = (inst >> 64) << 64;
    let mut seed: Vec<u8> = (halfr & halfl).to_le_bytes().to_vec();
    seed.append(&mut (halfr ^ halfl).to_le_bytes().to_vec());
    println!("passingDEC");
    state.aesdec(m256i::from_u8(&seed))*/
    let state: m128i = m128i::from_u128(inst);
    let halfr: u128 = (inst << 64) >> 64;
    let halfl: u128 = (inst >> 64) << 64;
    let state1: m128i = m128i::from_u128(halfr ^ halfl);
    let seed: u128 = halfr & halfl;
    //println!("passingDEC");
    let m1: m128i = state.aesdec(m128i::from_u128(seed));
    let m2: m128i = state1.aesdec(m128i::from_u128(seed));
    m256i::from_m128i(m1, m2)
}

pub fn add(inst: u128) -> m256i {
    let halfr: m256i = m256i::from_u128_one((inst << 64) >> 64);
    let halfl: m256i = m256i::from_u128_one((inst >> 64) << 64);
    //println!("passingADD");
    m256i::add(halfl, halfr)
}

pub fn sub(inst: u128) -> m256i {
    let halfr: m256i = m256i::from_u128_one((inst << 64) >> 64);
    let halfl: m256i = m256i::from_u128_one((inst >> 64) << 64);
    //println!("passingSUB");
    m256i::sub(halfl, halfr)
}

pub fn mul(inst: u128) -> m256i {
    let halfr: m256i = m256i::from_u128_one((inst << 64) >> 64);
    let halfl: m256i = m256i::from_u128_one((inst >> 64) << 64);
    //println!("passingMUL");
    m256i::mul(halfl, halfr)
}

pub fn div(inst: u128) -> m256d {
    let halfr: m256d = m256d::from_u128_one((inst << 64) >> 64);
    let halfl: m256d = m256d::from_u128_one((inst >> 64) << 64);
    //println!("passingDIV");
    m256d::div(halfl, halfr)
}

pub fn pal(inst: u128) -> m256i {
    let mut bytes: [u8; 16] = inst.to_le_bytes();
    let (halfl, halfr) = bytes.split_at_mut(8);
    halfr.reverse();

    let mut halfulll: Vec<u8> = halfl.to_vec();
    let mut halfullr: Vec<u8> = halfr.to_vec();
    halfulll.append(&mut halfl.to_vec());
    halfullr.append(&mut halfr.to_vec());

    if halfl == halfr {
        //println!("passingSYMPAL");
        m256i::palindrome_hell(m256i::from_u8(&halfulll), m256i::from_u8(&halfullr))
    } else {
        halfulll.reverse();
        //println!("passingASYMPAL");
        m256i::palindrome_hell(m256i::from_u8(&halfulll), m256i::from_u8(&halfullr))
    }
}