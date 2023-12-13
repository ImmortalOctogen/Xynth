//! Documentation is coming soon...
#![feature(stdsimd)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"),)//target_feature = "avx2") //all()
]

pub mod avx2;
pub mod opcode;
use opcode::*;


/// Documentation is coming soon...
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
/// Documentation is coming soon...
pub struct Registers {
    rwr0 : u128,
    rwr1 : u128,
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
                rwr0 : 0u128,
                rwr1 : 0u128,
            }
        }
    }

    pub fn hash(&mut self, keybytes: &[u8], length: HashLength) -> String {
        unsafe {self.init_memory(keybytes)}
        let mut buffer: Vec<u8> = Vec::<u8>::new();
        let mut counter: usize = 0usize;

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
            //ЗДЕСЬ
            match length {
                HashLength::H128 => {
                    let mut u8arr: Vec<u8> = self.memory[i].to_le_bytes().to_vec();
                    //println!("LEN {}", buffer.len());
                    if buffer.len() < 128 {
                        buffer.append(&mut u8arr);
                    } else {
                        if counter < 128/16 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let mut chunks: std::slice::ChunksMut<'_, u8> = buffer.chunks_mut(16);//.next().unwrap();
                        let mut chunk: &mut [u8] = &mut [0u8];
                        for _ in 0..counter {
                            //println!("CLOCKED");
                            chunk = chunks.next().unwrap()
                        }
                        //println!("{:?}", chunk);
                        //println!("{:?}", u8arr);
                        let merged: Vec<u8> = bitxor2vec(chunk.to_vec(), u8arr);
                        //println!("{:?}", merged);
                        for i in counter * 16 - 16..counter as usize * 16 {
                            //println!("LOL {}", i);
                            buffer[i] = merged[order];
                            order += 1usize;
                        }
                        //buffer.copy_from_slice(&merged[..16]);//copyarr.copy_from_slice(&merged[..16]);
                        //self.memory[i] = u128::from_le_bytes(copyarr);
                    }
                },
                HashLength::H256 => {
                    let mut u8arr: Vec<u8> = self.memory[i].to_le_bytes().to_vec();
                    //println!("LEN {}", buffer.len());
                    if buffer.len() < 256 {
                        buffer.append(&mut u8arr);
                    } else {
                        if counter < 256/16 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let mut chunks: std::slice::ChunksMut<'_, u8> = buffer.chunks_mut(16);//.next().unwrap();
                        let mut chunk: &mut [u8] = &mut [0u8];
                        for _ in 0..counter {
                            //println!("CLOCKED");
                            chunk = chunks.next().unwrap()
                        }
                        //println!("{:?}", chunk);
                        //println!("{:?}", u8arr);
                        let merged: Vec<u8> = bitxor2vec(chunk.to_vec(), u8arr);
                        //println!("{:?}", merged);
                        for i in counter * 16 - 16..counter as usize * 16 {
                            //println!("LOL {}", i);
                            buffer[i] = merged[order];
                            order += 1usize;
                        }
                        //buffer.copy_from_slice(&merged[..16]);//copyarr.copy_from_slice(&merged[..16]);
                        //self.memory[i] = u128::from_le_bytes(copyarr);
                    }
                },
                HashLength::H512 => {
                    let mut u8arr: Vec<u8> = self.memory[i].to_le_bytes().to_vec();
                    //println!("LEN {}", buffer.len());
                    if buffer.len() < 512 {
                        buffer.append(&mut u8arr);
                    } else {
                        if counter < 512/16 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let mut chunks: std::slice::ChunksMut<'_, u8> = buffer.chunks_mut(16);//.next().unwrap();
                        let mut chunk: &mut [u8] = &mut [0u8];
                        for _ in 0..counter {
                            //println!("CLOCKED");
                            chunk = chunks.next().unwrap()
                        }
                        //println!("{:?}", chunk);
                        //println!("{:?}", u8arr);
                        let merged: Vec<u8> = bitxor2vec(chunk.to_vec(), u8arr);
                        //println!("{:?}", merged);
                        for i in counter * 16 - 16..counter as usize * 16 {
                            //println!("LOL {}", i);
                            buffer[i] = merged[order];
                            order += 1usize;
                        }
                        //buffer.copy_from_slice(&merged[..16]);//copyarr.copy_from_slice(&merged[..16]);
                        //self.memory[i] = u128::from_le_bytes(copyarr);
                    }
                },
                HashLength::H1024 => {
                    let mut u8arr: Vec<u8> = self.memory[i].to_le_bytes().to_vec();
                    //println!("LEN {}", buffer.len());
                    if buffer.len() < 1024 {
                        buffer.append(&mut u8arr);
                    } else {
                        if counter < 1024/16 {
                            counter += 1
                        } else {
                            counter = 1;
                        }
                        let mut order: usize = 0usize;
                        let mut chunks: std::slice::ChunksMut<'_, u8> = buffer.chunks_mut(16);//.next().unwrap();
                        let mut chunk: &mut [u8] = &mut [0u8];
                        for _ in 0..counter {
                            //println!("CLOCKED");
                            chunk = chunks.next().unwrap()
                        }
                        //println!("{:?}", chunk);
                        //println!("{:?}", u8arr);
                        let merged: Vec<u8> = bitxor2vec(chunk.to_vec(), u8arr);
                        //println!("{:?}", merged);
                        for i in counter * 16 - 16..counter as usize * 16 {
                            //println!("LOL {}", i);
                            buffer[i] = merged[order];
                            order += 1usize;
                        }
                        //buffer.copy_from_slice(&merged[..16]);//copyarr.copy_from_slice(&merged[..16]);
                        //self.memory[i] = u128::from_le_bytes(copyarr);
                    }
                }
                /*_ => {
                    panic!()
                }*/
            }
        }
        return hex::encode(buffer)
    }

    #[target_feature(enable = "avx2")]
    unsafe fn init_memory(&mut self, seed: &[u8]) {
        let mem: &mut Box<[u128]> = &mut self.memory;
        let mut base: u128 = arrsum(seed);
        let mut revrs: u128;
        let mut secn: u128;

        for i in 0..mem.len() {
            revrs = 0u128;
            secn = base;
            while secn > 0 {
                //println!("Reversed number is {}", secn);
                revrs = (revrs << 3) + (revrs << 1) + secn % 10 ;
                secn /= 10;
            }
            base += revrs;
            mem[i] = base;
            //println!("Reversed number is {}", revrs);
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

fn bitxor2vec(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    //println!("ye");
    let mut result: Vec<u8> = Vec::<u8>::new();
    for (_, (aval, bval)) in a.iter().zip(&b).enumerate() {
        result.push(aval ^ bval);
    }
    result
}