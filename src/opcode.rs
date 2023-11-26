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
    let state: m256i = m256i::from_u128_one(inst);
    let halfr: u128 = (inst << 64) >> 64;
    let halfl: u128 = (inst >> 64) << 64;
    let seed: [u8; 16] = (halfr & halfl).to_ne_bytes();
    state.aesenc(m256i::from_u8(&seed))
}

pub fn dec(inst: u128) -> m256i {
    let state: m256i = m256i::from_u128_one(inst);
    let halfr: u128 = (inst << 64) >> 64;
    let halfl: u128 = (inst >> 64) << 64;
    let seed: [u8; 16] = (halfr & halfl).to_ne_bytes();
    state.aesdec(m256i::from_u8(&seed))
}

pub fn add(inst: u128) -> m256i {
    let halfr: m256i = m256i::from_u128_one((inst << 64) >> 64);
    let halfl: m256i = m256i::from_u128_one((inst >> 64) << 64);
    m256i::add(halfl, halfr)
}

pub fn sub(inst: u128) -> m256i {
    let halfr: m256i = m256i::from_u128_one((inst << 64) >> 64);
    let halfl: m256i = m256i::from_u128_one((inst >> 64) << 64);
    m256i::sub(halfl, halfr)
}

pub fn mul(inst: u128) -> m256i {
    let halfr: m256i = m256i::from_u128_one((inst << 64) >> 64);
    let halfl: m256i = m256i::from_u128_one((inst >> 64) << 64);
    m256i::mul(halfl, halfr)
}

pub fn div(inst: u128) -> m256d {
    let halfr: m256d = m256d::from_u128_one((inst << 64) >> 64);
    let halfl: m256d = m256d::from_u128_one((inst >> 64) << 64);
    m256d::div(halfl, halfr)
}

pub fn pal(inst: u128) -> m256i {
    let mut bytes: [u8; 16] = inst.to_ne_bytes();
    let (halfl, halfr) = bytes.split_at_mut(8);
    halfr.reverse();
    if halfl == halfr {
        m256i::palindrome_hell(m256i::from_u8(halfl), m256i::from_u8(halfr))
    } else {
        halfl.reverse();
        m256i::palindrome_hell(m256i::from_u8(halfl), m256i::from_u8(halfr))
    }
}