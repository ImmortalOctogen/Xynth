//! Documentation is coming soon...
#![feature(stdsimd)]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),)//target_feature = "avx2")
)]

pub mod avx2;
pub mod opcode;
use opcode::*;

/// Documentation is coming soon...
pub struct XynthVM {
    memory : [u128; 16777216],
    registers : Registers,
}

/// Documentation is coming soon...
pub struct Registers {
    rwr0 : u128,
    rwr1 : u128,
}

impl XynthVM {
    pub fn new() -> XynthVM {
        XynthVM {
            memory : [0u128; 16777216],
            registers : Registers {
                rwr0 : 0u128,
                rwr1 : 0u128,
            }
        }
    }

    pub fn hash(&mut self, keybytes: &[u8]) {
        unsafe {self.init_memory(keybytes)}

        for i in 0..self.memory.len() {
            self.registers.rwr0 = self.memory[i];
            self.registers.rwr1 = (self.memory[i] << 125) >> 125;
            let op = self.registers.rwr1;
            match op {
                op if op == OpCode::ADD as u128 => {
                    let a: u128 = add(self.registers.rwr0).as_u128();
                    self.memory[i] = a;
                },
                op if op == OpCode::SUB as u128 => {
                    let a: u128 = sub(self.registers.rwr0).as_u128();
                    self.memory[i] = a;
                },
                op if op == OpCode::MUL as u128 => {
                    let a: u128 = mul(self.registers.rwr0).as_u128();
                    self.memory[i] = a;
                },
                op if op == OpCode::DIV as u128 => {
                    let a: u128 = div(self.registers.rwr0).as_u128();
                    self.memory[i] = a;
                },
                op if op == OpCode::ENC as u128 => {
                    let a: u128 = enc(self.registers.rwr0).as_u128();
                    self.memory[i] = a;
                },
                op if op == OpCode::DEC as u128 => {
                    let a: u128 = dec(self.registers.rwr0).as_u128();
                    self.memory[i] = a;
                },
                op if op == OpCode::PAL as u128 => {
                    let a: u128 = pal(self.registers.rwr0).as_u128();
                    self.memory[i] = a;
                },
                _ => {
                    let a: u128 = add(self.registers.rwr0).as_u128();
                    let _b: u128 = enc(a).as_u128();
                    self.memory[i] = a;
                }
            }
        }
    }

    #[target_feature(enable = "avx2")]
    unsafe fn init_memory(&mut self, seed: &[u8]) {
        let mem: &mut [u128; 16777216] = &mut self.memory;
        let mut base: u128 = arrsum(seed);
        let mut revrs: u128;
        let mut secn: u128;

        for i in 0..mem.len() {
            revrs = 0u128;
            secn = base;
            while secn > 0 {
                println!("Reversed number is {}", secn);
                revrs = (revrs << 3) + (revrs << 1) + secn % 10 ;
                secn /= 10;
            }
            base += revrs;
            mem[i] = base;
            println!("Reversed number is {}", revrs);
        }
    }
}

fn arrsum(arr: &[u8]) -> u128 {
    let mut sum: u128 = 0u128;
    
    for i in 0..arr.len() {
     sum += arr[i] as u128;	
    }
    sum
}
