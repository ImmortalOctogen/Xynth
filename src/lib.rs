//! XynthVM hashing algo, that runs large program. Program code depends on input data.
//! XynthVM is developed for same purpose, as RandomX - to avoid running algo on GPU and ASIC.
//#![feature(stdsimd)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"),)//target_feature = "avx2") //all()
]

mod avx2;
mod opcode;

use avx2::*;
use opcode::*;

/// VM, that proccessing 256 MiB random executable virtual code on 1 thread for now (Almost like RandomX).
/// VM contains 6 registers: 2 for integer, 2 for float, 2 for aes integer-like data.
/// 
/// Why 256MiB code? Well, for maximum avalanche effect.
pub struct XynthVM {
    memory : Box<[u128]>,//[u128; 16777216],
    registers : Registers,
}

#[derive(PartialEq, Clone, Copy)]
pub enum HashLength {
    H128,
    H256,
    H512,
    H1024,
}

struct Registers {
    rwr0 : m256i,
    rwr1 : m256i,
    rwr2 : m256d,
    rwr3 : m256d,
    aes0 : m128i,
    aes1 : m128i,
}

impl Default for XynthVM {
    fn default() -> Self {
        Self::new()
    }
}

impl XynthVM {
    pub fn new() -> XynthVM {
        XynthVM {
            memory : [0u128; 16777216].to_vec().into_boxed_slice(), //256 MiB
            registers : Registers {
                rwr0 : m256i::zero(),
                rwr1 : m256i::zero(),
                rwr2 : m256d::zero(),
                rwr3 : m256d::zero(),
                aes0 : m128i::zero(),
                aes1 : m128i::zero(),
            }
        }
    }

    /// # Safety
    /// Runs only on AVX2 CPUs
    #[target_feature(enable = "avx2")]
    pub unsafe fn hash(&mut self, keybytes: &[u8], length: HashLength, salt: Option<&[u8]>) -> Vec<u8> {
        if salt.is_some() {
            self.init_memory(&bitxor2vec(keybytes, salt.unwrap_unchecked()));
        } else {
            self.init_memory(keybytes);
        }
        let mut buffer: Vec<u8> = Vec::<u8>::new();
        let regs: &mut Registers = &mut self.registers;
        let mut counter: usize = 0usize;
        for i in self.memory.iter_mut() {
            let op: u128 = (*i << 125) >> 125;
            match op {
                op if op == OpCode::ADD as u128 => {
                    regs.rwr0 = m256i::from_u128_one((*i << 64) >> 64);
                    regs.rwr1 = m256i::from_u128_one((*i >> 64) << 64);
                    add(regs);
                    *i = regs.rwr0.as_u128();
                },
                op if op == OpCode::SUB as u128 => {
                    regs.rwr0 = m256i::from_u128_one((*i << 64) >> 64);
                    regs.rwr1 = m256i::from_u128_one((*i >> 64) << 64);
                    sub(regs);
                    *i = regs.rwr0.as_u128();
                },
                op if op == OpCode::MUL as u128 => {
                    regs.rwr0 = m256i::from_u128_one((*i << 64) >> 64);
                    regs.rwr1 = m256i::from_u128_one((*i >> 64) << 64);
                    mul(regs);
                    *i = regs.rwr0.as_u128();
                },
                op if op == OpCode::DIV as u128 => {
                    regs.rwr2 = m256d::from_u128_one((*i << 64) >> 64);
                    regs.rwr3 = m256d::from_u128_one((*i >> 64) << 64);
                    div(regs);
                    *i = regs.rwr2.as_u128();
                },
                op if op == OpCode::ENC as u128 => {
                    /*self.registers.rwr0 = m256i::from_u128_one((self.memory[i] << 64) >> 64);
                    self.registers.rwr1 = m256i::from_u128_one((self.memory[i] >> 64) << 64);*/
                    regs.aes0 = m128i::from_u128(*i);
                    regs.aes1 = m128i::from_u128(((*i << 64) >> 64) ^ ((*i >> 64) << 64));
                    enc(regs);
                    *i = regs.rwr0.as_u128();
                },
                op if op == OpCode::DEC as u128 => {
                    regs.aes0 = m128i::from_u128(*i);
                    regs.aes1 = m128i::from_u128(((*i << 64) >> 64) ^ ((*i >> 64) << 64));
                    dec(regs);
                    *i = regs.rwr0.as_u128();
                },
                op if op == OpCode::PAL as u128 => {
                    regs.rwr0 = m256i::from_u128_one((*i << 64) >> 64);
                    regs.rwr1 = m256i::from_u128_one((*i >> 64) << 64);
                    pal(regs);
                    *i = regs.rwr0.as_u128();
                },
                op if op == OpCode::SQR as u128 => {
                    sqrt(regs);
                    *i = regs.rwr2.as_u128();
                },
                _ => {
                    regs.rwr0 = m256i::from_u128_one((*i << 64) >> 64);
                    regs.rwr1 = m256i::from_u128_one((*i >> 64) << 64);
                    add(regs);
                    *i = regs.rwr0.as_u128();
                },
            }
            match length {
                HashLength::H128 => {
                    let mut u8arr: Vec<u8> = i.to_le_bytes().to_vec();
                    buffer.reserve_exact(16);
                    if buffer.len() < 16 {
                        buffer.append(&mut u8arr);
                    } else {
                        if counter < 1 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let mut chunks: std::slice::ChunksExact<'_, u8> = buffer.chunks_exact(16);//.next().unwrap();
                        let chunk: &[u8] = chunks.nth(counter-1).unwrap_unchecked();
                        /*for _ in 0..counter {
                            chunk = chunks.next().unwrap()
                        }*/

                        let mut merged: Vec<u8> = bitxor2vec(chunk, &u8arr);

                        for i in merged.iter_mut().enumerate() {
                            *i.1 ^= u8arr[i.0]
                        }
                        for i in buffer.iter_mut().take(counter * 16).skip(counter * 16 - 16) {
                            *i = merged[order];
                            order += 1usize;
                        }
                    }
                },
                HashLength::H256 => {
                    let mut u8arr: Vec<u8> = i.to_le_bytes().to_vec();
                    buffer.reserve_exact(32);
                    if buffer.len() < 32 {
                        buffer.append(&mut u8arr);
                    } else {
                        if counter < 2 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let mut chunks: std::slice::ChunksExact<'_, u8> = buffer.chunks_exact(16);//.next().unwrap();
                        let chunk: &[u8] = chunks.nth(counter-1).unwrap_unchecked();
                        /*for _ in 0..counter {
                            chunk = chunks.next().unwrap()
                        }*/

                        let mut merged: Vec<u8> = bitxor2vec(chunk, &u8arr);

                        for i in merged.iter_mut().enumerate() {
                            *i.1 ^= u8arr[i.0]
                        }
                        for i in buffer.iter_mut().take(counter * 16).skip(counter * 16 - 16) {
                            *i = merged[order];
                            order += 1usize;
                        }
                    }
                },
                HashLength::H512 => {
                    let mut u8arr: Vec<u8> = i.to_le_bytes().to_vec();
                    buffer.reserve_exact(64);
                    if buffer.len() < 64 {
                        buffer.append(&mut u8arr);
                    } else {
                        if counter < 4 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let mut chunks: std::slice::ChunksExact<'_, u8> = buffer.chunks_exact(16);//.next().unwrap();
                        let chunk: &[u8] = chunks.nth(counter-1).unwrap_unchecked();
                        /*for _ in 0..counter {
                            chunk = chunks.next().unwrap()
                        }*/

                        let mut merged: Vec<u8> = bitxor2vec(chunk, &u8arr);

                        for i in merged.iter_mut().enumerate() {
                            *i.1 ^= u8arr[i.0]
                        }
                        for i in buffer.iter_mut().take(counter * 16).skip(counter * 16 - 16) {
                            *i = merged[order];
                            order += 1usize;
                        }
                    }
                },
                HashLength::H1024 => {
                    let mut u8arr: Vec<u8> = i.to_le_bytes().to_vec();
                    buffer.reserve_exact(128);
                    if buffer.len() < 128 {
                        buffer.append(&mut u8arr);
                    } else {
                        if counter < 8 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let mut chunks: std::slice::ChunksExact<'_, u8> = buffer.chunks_exact(16);//.next().unwrap();
                        let chunk: &[u8] = chunks.nth(counter-1).unwrap_unchecked();
                        /*for _ in 0..counter {
                            chunk = chunks.next().unwrap()
                        }*/

                        let mut merged: Vec<u8> = bitxor2vec(chunk, &u8arr);

                        for i in merged.iter_mut().enumerate() {
                            *i.1 ^= u8arr[i.0]
                        }
                        for i in buffer.iter_mut().take(counter * 16).skip(counter * 16 - 16) {
                            *i = merged[order];
                            order += 1usize;
                        }
                    }
                }
            }
        }
        //hex::encode(buffer)
        buffer
    }

    #[target_feature(enable = "avx2")]
    unsafe fn init_memory(&mut self, seed: &[u8]) {
        //let mem: &mut Box<[u128]> = &mut self.memory;
        let mut base: u128 = arrsum(seed);
        let mut revrs: u128;
        let mut secn: u128;

        for i in 0..self.memory.len() {
            revrs = 0u128;
            secn = base;
            while secn > 0 {
                revrs = (revrs << 3) + (revrs << 1) + secn % 10 ;
                secn /= 10;
            }
            base = base.wrapping_add(revrs); //АНТИПЕРЕПОЛНЕНИЕ u128. СТАРЫЙ: base += revrs;
            self.memory[i] = base;
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn arrsum(arr: &[u8]) -> u128 {
    let mut sum: u128 = 0u128;
    
    for i in arr {
        sum = sum.wrapping_add(*i as u128);
    }
    sum
}

#[target_feature(enable = "avx2")]
unsafe fn bitxor2vec(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::<u8>::with_capacity(16);
    let mut buf: u8;

    for (aval, bval) in a.iter().zip(b) {
        //result.push(aval ^ bval);
        buf = aval ^ bval;
        result.push((buf >> 4) + (buf << 4));
    }
    for i in 0..result.len() - 1 {
        result[i] ^= result[i+1];
        //u ^= result[i];
    }

    let u: u8 = result[0]^result.last().unwrap();

    if let Some(last) = result.last_mut() {
        *last ^= u;
    }
    result
}