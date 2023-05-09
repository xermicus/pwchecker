#![no_main]
//#![no_std] // std support is experimental

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let pw: String = env::read();

    assert!(pw.chars().any(|c| c.is_ascii_punctuation()));

    env::commit(Impl::hash_bytes(pw.as_bytes()));
}
