// Tests assert by panicking and print measurements, so those lints are off here only.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

//! The four pinned 37-limb wallet vectors, proved again by the prover this core carries.
//! `PROD_VECTORS` is their folder. Each proof and its shared form must come out byte for byte,
//! and the shared form must cut to the single-call layout. That layout and the 13 words are
//! written to `PROD_VECTORS_OUT`, for a check against the production pool's own verifier.

use nox_shield_core::prover::launch::publics::{pool_words, LIMBS};
use nox_shield_core::wallet::one_call;

fn read(dir: &str, name: &str, file: &str) -> Vec<u8> {
    std::fs::read(format!("{dir}/{name}/{file}")).expect(file)
}

fn unhex(text: &str) -> Vec<u8> {
    let t = text.trim().trim_start_matches("0x");
    (0..t.len()).step_by(2).map(|i| u8::from_str_radix(&t[i..i + 2], 16).expect("hex")).collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
#[ignore = "needs the pinned vectors on disk, and proves four times"]
fn the_four_pinned_production_vectors_prove_byte_for_byte() {
    let dir = std::env::var("PROD_VECTORS").expect("PROD_VECTORS");
    let out = std::env::var("PROD_VECTORS_OUT").expect("PROD_VECTORS_OUT");
    for name in ["transfer-eth", "withdraw-eth", "transfer-nox", "withdraw-nox"] {
        let text = |f: &str| String::from_utf8(read(&dir, name, f)).expect(f);
        let entropy = unhex(&text("entropy.hex"));
        let (proof, cache) =
            nox_prover::prove_keeping_cache(&text("request.json"), &text("seed.json"), &entropy)
                .expect("a proof");
        assert_eq!(proof.bytes, read(&dir, name, "proof.bin"), "{name}: the proof");
        let shared = nox_prover::to_shared(&proof.bytes, &proof.publics, &cache).expect("shared");
        assert_eq!(shared, read(&dir, name, "proof-format7.bin"), "{name}: the shared form");
        nox_prover::verify_shared(&shared, &proof.publics, &cache).expect("it verifies");
        let limbs: [u64; LIMBS] = proof.publics.clone().try_into().expect("37 limbs");
        let whole = one_call(&shared).expect("it cuts");
        let words: Vec<String> =
            pool_words(&limbs).iter().map(|w| format!("0x{}", hex(w))).collect();
        std::fs::write(format!("{out}/{name}.whole"), format!("0x{}", hex(&whole))).expect("out");
        std::fs::write(format!("{out}/{name}.words"), format!("[{}]", words.join(",")))
            .expect("out");
        println!(
            "{name}: {} bytes, byte for byte, {} in the single call",
            shared.len(),
            whole.len()
        );
    }
}
