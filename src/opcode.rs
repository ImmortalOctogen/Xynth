use crate::avx2::*;
use crate::Registers;

#[derive(PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub (crate) enum OpCode {
    ENC = 0b000,
    DEC = 0b001,
    ADD = 0b010,
    SUB = 0b011,
    DIV = 0b100,
    MUL = 0b101,
    PAL = 0b110,
    SQR = 0b111,
}

#[target_feature(enable = "avx2")]
pub (crate) unsafe fn enc(regs: &mut Registers) {
    regs.rwr0 = m256i::and(regs.rwr0, regs.rwr1);
    regs.aes0 = regs.aes0.aesenc(regs.rwr0.as_m128i());
    regs.aes1 = regs.aes1.aesenc(regs.rwr0.as_m128i());
    regs.rwr0 = m256i::from_m128i(regs.aes0, regs.aes0);
}

#[target_feature(enable = "avx2")]
pub (crate) unsafe fn dec(regs: &mut Registers) {
    regs.rwr0 = m256i::and(regs.rwr0, regs.rwr1);
    regs.aes0 = regs.aes0.aesdec(regs.rwr0.as_m128i());
    regs.aes1 = regs.aes1.aesdec(regs.rwr0.as_m128i());
    regs.rwr0 = m256i::from_m128i(regs.aes0, regs.aes0);
}

#[target_feature(enable = "avx2")]
pub (crate) unsafe fn add(regs: &mut Registers) {
    regs.rwr0 = m256i::add(regs.rwr0, regs.rwr1)
}

#[target_feature(enable = "avx2")]
pub (crate) unsafe fn sub(regs: &mut Registers) {
    regs.rwr0 = m256i::sub(regs.rwr0, regs.rwr1)
}

#[target_feature(enable = "avx2")]
pub (crate) unsafe fn mul(regs: &mut Registers) {
    regs.rwr0 = m256i::mul(regs.rwr0, regs.rwr1)
}

#[target_feature(enable = "avx2")]
pub (crate) unsafe fn div(regs: &mut Registers) {
    regs.rwr2 = m256d::div(regs.rwr2, regs.rwr3)
}

#[target_feature(enable = "avx2")]
pub (crate) unsafe fn sqrt(regs: &mut Registers) {
    regs.rwr2.sqrt()
}

#[target_feature(enable = "avx2")]
pub (crate) unsafe fn pal(regs: &mut Registers) {
    let mut halfullr: Vec<u8> = regs.rwr0.as_u128().to_le_bytes().to_vec();
    let mut halfulll: Vec<u8> = regs.rwr1.as_u128().to_le_bytes().to_vec();
    halfulll.append(&mut halfullr.clone());
    halfullr.append(&mut reverse(regs.rwr1.as_u128()).to_le_bytes().to_vec());

    if regs.rwr1.as_u128() == reverse(regs.rwr0.as_u128()) {
        regs.rwr0 = m256i::palindrome_hell(m256i::from_u8(&halfulll), m256i::from_u8(&halfullr));
    } else {
        halfulll.reverse();
        regs.rwr0 = m256i::palindrome_hell(m256i::from_u8(&halfulll), m256i::from_u8(&halfullr));
    }
}

#[target_feature(enable = "avx2")]
unsafe fn reverse(mut num: u128) -> u128 {
    let mut revrs: u128 = 0u128;
    while num > 0 {
        revrs = (revrs << 3) + (revrs << 1) + num % 10;
        num /= 10;
    }
    num += revrs;
    num
}