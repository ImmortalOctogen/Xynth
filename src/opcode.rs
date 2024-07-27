use crate::{
    avx2::*,
    XynthVM, Registers, WORD,
};
use std::slice::from_raw_parts_mut;

#[derive(PartialEq)]
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
pub (crate) enum OpCode { // 11/16 opcodes
    ENC = 0b0000,
    DEC = 0b0001,
    ADD = 0b0010,
    SUB = 0b0011,
    DIV = 0b0100,
    MUL = 0b0101,
    PAL = 0b0110, // palindrome :p why? idk
    SQR = 0b0111,
    MWR = 0b1000, // mem write
    MRE = 0b1001, // mem read
    STR = 0b1010, // store in reg
}

pub (crate) unsafe fn enc(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;
    regs.rwr0 = m256i::and(regs.rwr0, regs.rwr1);
    regs.aes0 = regs.aes0.aesenc(regs.rwr0.as_m128i());
    regs.aes1 = regs.aes1.aesenc(regs.rwr0.as_m128i());
    regs.rwr0 = m256i::from_m128i(regs.aes0, regs.aes0);
}

pub (crate) unsafe fn dec(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;
    regs.rwr0 = m256i::and(regs.rwr0, regs.rwr1);
    regs.aes0 = regs.aes0.aesdec(regs.rwr0.as_m128i());
    regs.aes1 = regs.aes1.aesdec(regs.rwr0.as_m128i());
    regs.rwr0 = m256i::from_m128i(regs.aes0, regs.aes0);
}

pub (crate) unsafe fn add(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;
    regs.rwr0 = m256i::add(regs.rwr0, regs.rwr1)
}

pub (crate) unsafe fn sub(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;
    regs.rwr0 = m256i::sub(regs.rwr0, regs.rwr1)
}

pub (crate) unsafe fn mul(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;
    regs.rwr0 = m256i::mul(regs.rwr0, regs.rwr1)
}

pub (crate) unsafe fn div(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;
    regs.rwr2 = m256d::div(regs.rwr2, regs.rwr3)
}

/*#[target_feature(enable = "avx2")]
pub (crate) unsafe fn sqrt(regs: &mut Registers) {
    regs.rwr2.sqrt()
}*/

pub (crate) unsafe fn pal(vm: &mut XynthVM) {
    /*let mut halfullr: Vec<u8> = regs.rwr0.as_u128().to_le_bytes().to_vec();
    let mut halfulll: Vec<u8> = regs.rwr1.as_u128().to_le_bytes().to_vec();
    halfulll.append(&mut halfullr.clone());
    halfullr.append(&mut reverse(regs.rwr1.as_u128()).to_le_bytes().to_vec());*/
    let regs: &mut Registers = &mut vm.registers;

    let right: u128 = regs.rwr0.as_u128();
    let left: u128 = regs.rwr1.as_u128();

    if left == reverse(right) {
        regs.rwr0 = m256i::palindrome_hell(m256i::from_u128_one(left), m256i::from_u128_one(right))
    } else {
        //halfulll.reverse();
        regs.rwr0 = m256i::zero()
    }
}

pub (crate) unsafe fn sqr(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;

    match ((WORD << 123) >> 127) as u8 {
        0 => regs.rwr2.sqrt(),
        _ => regs.rwr3.sqrt(),
    }
}

pub (crate) unsafe fn mwr(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;
    let u256ptr: &mut [[u128; 2]] = {
        let mem: &[u128] = &vm.memory[2_097_152..];
        from_raw_parts_mut(mem.as_ptr() as *mut [u128; 2], mem.len())
    };

    match ((((WORD << 100) >> 104).saturating_sub(2097152)/2) as usize, ((WORD << 99) >> 127) as u8) {
        (v, 0) => u256ptr[v] = regs.rwr0.as_u128x2(),
        (v, 1) => u256ptr[v] = regs.rwr1.as_u128x2(),//self.memory[v] = regs.rwr1.as_u128(),
        (v, 2) => u256ptr[v] = regs.rwr2.as_u128x2(),
        (v, 3) => u256ptr[v] = regs.rwr3.as_u128x2(),
        (_, _) => {}
    }
}

pub (crate) unsafe fn mre(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;
    let u256ptr: &mut [[u128; 2]] = {
        let mem: &[u128] = &vm.memory[2_097_152..];
        from_raw_parts_mut(mem.as_ptr() as *mut [u128; 2], mem.len())
    };

    match ((((WORD << 100) >> 104).saturating_sub(2097152)/2) as usize, ((WORD << 99) >> 127) as u8) {
        (v, 0) => regs.rwr0 = m256i::from_u128x2(u256ptr[v]), //self.memory[v] = regs.rwr0.as_u128(),
        (v, 1) => regs.rwr1 = m256i::from_u128x2(u256ptr[v]),
        (v, 2) => regs.rwr2 = m256d::from_u128x2(u256ptr[v]),
        (v, 3) => regs.rwr3 = m256d::from_u128x2(u256ptr[v]),
        (_, _) => {}
    }
}

pub (crate) unsafe fn store(vm: &mut XynthVM) {
    let regs: &mut Registers = &mut vm.registers;

    match (((WORD << 37) >> 42), ((WORD << 99) >> 127) as u8) {
        (v, 0) => regs.rwr0 = m256i::from_u128_one(v), //self.memory[v] = regs.rwr0.as_u128(),
        (v, 1) => regs.rwr1 = m256i::from_u128_one(v),
        (v, 2) => regs.rwr2 = m256d::from_u128_one(v),
        (v, 3) => regs.rwr3 = m256d::from_u128_one(v),
        (_, _) => {}
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