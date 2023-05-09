use std::{io, time::Instant};

use methods::{PW_CHECK_ELF, PW_CHECK_ID};
use risc0_zkp::core::sha::Digest;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};

fn main() {
    // Make the prover.
    let mut prover =
        Prover::new(PW_CHECK_ELF).expect("Prover should be constructed from valid ELF binary");

    let mut pw = String::new();
    io::stdin().read_line(&mut pw).unwrap();
    let input = to_vec(&pw[..pw.len() - 1]).unwrap();
    prover.add_input_u32_slice(&input);

    // Run prover & generate receipt
    let now = Instant::now();
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");
    println!("Proof generation took in {}ms", now.elapsed().as_millis());

    let now = Instant::now();
    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(&PW_CHECK_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    println!("Proof verification took in {}ms", now.elapsed().as_millis());
    println!("Proof size: {}kb", receipt.get_seal_bytes().len() / 1000);

    let journal = receipt.get_journal_bytes();
    let output: Digest = from_slice(&journal).unwrap();
    println!("Password hash contains punctuation characters: {}", output);

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here
}
