# Xynth
[![Discord](https://img.shields.io/discord/1143160291773927494?style=flat&logo=discord&logoColor=00ECFF&label=discord&link=discord.gg%2FuRRzzftdCw)](https://discord.gg/2FuRRzzftdCw)
[![Crates.io Version](https://img.shields.io/crates/v/xynth)](https://crates.io/crates/xynth)
[![docs.rs](https://img.shields.io/docsrs/xynth?style=flat&link=docs.rs%2Fxynth)](https://docs.rs/xynth)

- My profiles
    - RetroShare ID - 
ABCOR8VE42d6yImJdqnlE6FaAxRQvoT4dVgT/ibTYa0o61xRAh3epQEJT2N0b2Nyb3NzkEIAAAAEAAAyNjVob2RtNWU0bHhkcjJw
dWZ3cmZyY3I1Z2xnZWhmMnhlYW9jMmJ3and0Z2J1MmtvbHphLmIzMi5pMnAEA5YMNg==
    - [Tox.chat](https://tox.chat) - B4444ACFE6E42A72F67879B13B8D758CD6BCD1F498E7AA882A5680FDE2B31D5CE07EEF119CD9

> [!IMPORTANT]
> The resources listed above are mostly written in Russian

# [ENG]

Xynth is a hashing algorithm, that runs large program on virtual VLIW (128 bit) CPU machine. Program code depends on input data.
Xynth is developed for the same purpose, as RandomX - to avoid processing algo on GPU and ASIC.

`master` - latest release

> [!CAUTION]
> This algorithm has not been tested using cryptanalysis

## Requirements
- **x64 CPU with AVX2 & AES support**

# [RU]

Xynth - это алгоритм хэширования, который запускает большую программу на виртуальном VLIW (128-битном) процессоре. Программный код зависит от входных данных.
Xynth разработан с той же целью, что и RandomX - чтобы избежать обрабатывания алгоритма на GPU и ASIC.

`master` - последний релиз

> [!CAUTION]
> Этот алгоритм не был протестирован с помощью криптоанализа

## Требования
- **Процессор x64 с поддержкой AVX2 и AES**

# Example
```rust
use xynth::XynthVM;
fn main() {
    let mut m: XynthVM = XynthVM::new();
    let a;
    let b;
    unsafe {
        a = m.hash(&1i32.to_le_bytes(), xynth::HashLength::H256, None);
        b = m.hash(&2i32.to_le_bytes(), xynth::HashLength::H256, None);
    }
    assert!(a != b)
}

```

# MSRV
Latest rustc night build :c

# The Path of the Xynth
- [ ] Multithreading
- [ ] Cryptanalysis
- [ ] ~~Learn how to use Git~~

# Versioning
- 0.0.x - patch
- 0.x.0 - release with a change in the output hash
- x.0.0 - full algorithm change