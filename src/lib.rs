//! Xynth hashing algorithm, that runs large program on virtual VLIW (128 bit) CPU machine. Program code depends on input data.
//! Xynth is developed for the same purpose, as RandomX - to avoid processing algo on GPU and ASIC.
//! # Examples
//! ```
//! let a = vec![1, 2, 3];
//! assert_eq!(a.len(), 3);
//! ```
//#![feature(stdsimd)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))//, target_feature = "avx2") //all()
]

mod avx2;
mod opcode;

use avx2::*;
use opcode::*;
use std::{ptr, slice::from_raw_parts};

static mut WORD: u128 = 0u128;
//static mut PTR: &mut [[u128; 2]] = &mut [];
const OPCODES: [unsafe fn(&mut XynthVM); 11] = [enc, dec, add, sub, div, mul, pal, sqr, mwr, mre, store];

/// VM, that proccessing 32 MiB random executable virtual code on 1 thread and uses 224 MiB as RAM.
/// VM contains 6 registers: 2 for integer, 2 for float, 2 for AES integer data.
pub struct XynthVM {
    memory : Box<[u128]>,//[u128; 16777216],
    registers : Registers,
}

/// Length of hash in bits. 128b = 16 bytes. Length does not affect on perfomance, BUT on output result.
#[derive(PartialEq, Clone, Copy)]
pub enum HashLength {
    H128, //16
    H256, //32
    H512, //64
    H1024, //128 O.O
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
    /// Create and allocate XynthVM
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
        let mut buffer: Vec<u8> = {
            match length {
                HashLength::H128 => Vec::<u8>::with_capacity(16),
                HashLength::H256 => Vec::<u8>::with_capacity(32),
                HashLength::H512 => Vec::<u8>::with_capacity(64),
                HashLength::H1024 => Vec::<u8>::with_capacity(128),
            }
        };
        //let regs: &mut Registers = &mut self.registers;
        let mut counter: usize = 0usize;

        // 256 битов для AVX2
        /*let u256ptr: &mut [[u128; 2]] = {
            let mem: &[u128] = &self.memory[2_097_152..];
            from_raw_parts_mut(mem.as_ptr() as *mut [u128; 2], mem.len())
        };*/

        for p in 0usize..2_097_152_usize { //.iter_mut() 32 MiB as index (exclude range)
            //let i: u128 = self.memory[p];
            WORD = self.memory[p];
            let op: usize = ((WORD << 124) >> 124) as usize;

            if let 0..=10 = op { // Call Threading
                OPCODES[op](self)
            }

            match length {
                HashLength::H128 => {
                    if buffer.len() < 16 {
                        /*let mut u8arr: Vec<u8> = WORD.to_le_bytes().to_vec();
                        buffer.append(&mut u8arr);*/
                        let u8arr: [u8; 16] = WORD.to_le_bytes();
                        append_fixed(&mut buffer, u8arr);
                    } else {
                        let u8arr: [u8; 16] = WORD.to_le_bytes();
                        if counter < 1 {
                            counter += 1
                        }
                        let mut order: usize = 0usize;
                        /*let mut chunks: ChunksExact<'_, u8> = buffer.chunks_exact(16);//.next().unwrap();
                        let chunk: &[u8] = chunks.nth(counter-1).unwrap_unchecked();*/
                        let chunk: [u8; 16] = {
                            from_raw_parts(buffer.as_ptr() as *mut [u8; 16], buffer.len())[counter-1]
                        };
                        /*for _ in 0..counter {
                            chunk = chunks.next().unwrap()
                        }*/

                        let merged: [u8; 16] = bitxor2u8x16(chunk, u8arr);

                        // Степень бреда и не нужности такой же, как и в bitxor2vec
                        // Убираем, потому что: a ^ b = c, c ^ b = a...
                        /*for i in merged.iter_mut().enumerate() {
                            *i.1 ^= u8arr[i.0]
                        }
                        // ИЛИ
                        for i in 0..merged.len() {
                            merged[i] ^= u8arr[i]
                        }*/

                        for i in buffer.iter_mut().take(counter * 16).skip(counter * 16 - 16) {
                            *i = merged[order];
                            order += 1usize;
                        }
                    }
                },
                HashLength::H256 => {
                    if buffer.len() < 32 {
                        let u8arr: [u8; 16] = WORD.to_le_bytes();
                        append_fixed(&mut buffer, u8arr);
                    } else {
                        let u8arr: [u8; 16] = WORD.to_le_bytes();
                        if counter < 2 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let chunk: [u8; 16] = {
                            from_raw_parts(buffer.as_ptr() as *mut [u8; 16], buffer.len())[counter-1]
                        };
                        /*for _ in 0..counter {
                            chunk = chunks.next().unwrap()
                        }*/

                        let merged: [u8; 16] = bitxor2u8x16(chunk, u8arr);

                        /*for i in merged.iter_mut().enumerate() {
                            *i.1 ^= u8arr[i.0]
                        }*/
                        for i in buffer.iter_mut().take(counter * 16).skip(counter * 16 - 16) {
                            *i = merged[order];
                            order += 1usize;
                        }
                    }
                },
                HashLength::H512 => {
                    if buffer.len() < 64 {
                        let u8arr: [u8; 16] = WORD.to_le_bytes();
                        append_fixed(&mut buffer, u8arr);
                    } else {
                        let u8arr: [u8; 16] = WORD.to_le_bytes();
                        if counter < 4 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let chunk: [u8; 16] = {
                            from_raw_parts(buffer.as_ptr() as *mut [u8; 16], buffer.len())[counter-1]
                        };
                        /*for _ in 0..counter {
                            chunk = chunks.next().unwrap()
                        }*/

                        let merged: [u8; 16] = bitxor2u8x16(chunk, u8arr);

                        /*for i in merged.iter_mut().enumerate() {
                            *i.1 ^= u8arr[i.0]
                        }*/
                        for i in buffer.iter_mut().take(counter * 16).skip(counter * 16 - 16) {
                            *i = merged[order];
                            order += 1usize;
                        }
                    }
                },
                HashLength::H1024 => {
                    if buffer.len() < 128 {
                        let u8arr: [u8; 16] = WORD.to_le_bytes();
                        append_fixed(&mut buffer, u8arr);
                    } else {
                        let u8arr: [u8; 16] = WORD.to_le_bytes();
                        if counter < 8 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let chunk: [u8; 16] = {
                            from_raw_parts(buffer.as_ptr() as *mut [u8; 16], buffer.len())[counter-1]
                        };
                        /*for _ in 0..counter {
                            chunk = chunks.next().unwrap()
                        }*/

                        let merged: [u8; 16] = bitxor2u8x16(chunk, u8arr);

                        /*for i in merged.iter_mut().enumerate() {
                            *i.1 ^= u8arr[i.0]
                        }*/
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

        /*for i in 0..self.memory.len() {
            revrs = 0u128;
            secn = base;
            while secn > 0 {
                revrs = (revrs << 3) + (revrs << 1) + secn % 10 ;
                secn /= 10;
            }
            base = base.wrapping_add(revrs); //АНТИПЕРЕПОЛНЕНИЕ u128. СТАРЫЙ: base += revrs;
            self.memory[i] = base;
        }*/

        for i in &mut self.memory {
            revrs = 0u128;
            secn = base;
            while secn > 0 {
                revrs = (revrs << 3) + (revrs << 1) + secn % 10 ;
                secn /= 10;
            }
            base = base.wrapping_add(revrs);
            *i = base;
        }
    }
}

#[target_feature(enable = "avx2")]
const unsafe fn arrsum(arr: &[u8]) -> u128 {
    let mut sum: u128 = 0u128;
    let mut clock: usize = 0usize;
    
    /*for i in arr {
        sum = sum.wrapping_add(*i as u128);
    }*/
    while clock < arr.len() {
        sum = sum.wrapping_add(arr[clock] as u128);
        clock += 1
    }
    sum
}

#[target_feature(enable = "avx2")]
unsafe fn bitxor2vec(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::<u8>::with_capacity(a.len().abs_diff(b.len())+a.len());
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

    // ХУЙНЯ: result[result.len()] ^ result[result.len()] == 0, следовательно, бесполезно
    // По факту, мы таким образом сокращаем result на 1 элемент
    /*let u: u8 = result[0]^result.last().unwrap();

    if let Some(last) = result.last_mut() {
        *last ^= u;
    }*/
    result
}

#[target_feature(enable = "avx2")]
const unsafe fn bitxor2u8x16(mut a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let mut clock: usize = 0usize;

    while clock < 16 {
        a[clock] ^= b[clock];
        clock += 1
    }
    clock = 0;
    while clock < 15 {
        a[clock] ^= a[clock+1];
        clock += 1
    }

    a
}

unsafe fn append_fixed(that: &mut Vec<u8>, other: [u8; 16]) {
    // ЭКВИВАЛЕНТ that.extend_from_slice(&other), но с более конкретными значениями
    // ПЕРЕД ИСПОЛЬЗОВАНИЕМ ЭТОЙ ФУНКЦИИ, "that" ДОЛЖЕН УЖЕ ИМЕТЬ ДОСТАТОЧНО ПРОСТРАНСТВА!!!
    core::hint::assert_unchecked(that.capacity() >= that.len());
    let mut clock: usize = 0usize;
    while clock < 16 {
        let len: usize = that.len();
        //UNSAFE
            ptr::write(that.as_mut_ptr().add(len), other[clock]);
            clock += 1;
            that.set_len(len + 1);
    }
}
