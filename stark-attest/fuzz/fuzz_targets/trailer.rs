// The gate under fire. The Lean development proves the parse total over its
// model; this holds the code itself to the same standard, on inputs no test
// author would write. Any panic, overflow or out of bounds read here is a
// finding against the entry point every consumer runs on attacker bytes.
#![no_main]
use libfuzzer_sys::fuzz_target;
use nonos_stark::air::{verify_attestation_trailer, Poseidon, RATE};
use nonos_stark::attest_params::{LOG_ROUNDS, N_QUERIES};
use nonos_stark::field::Fp;

fuzz_target!(|data: &[u8]| {
    if data.len() < 40 {
        return;
    }
    let mut root = [Fp::ZERO; RATE];
    for (i, lane) in root.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&data[i * 8..i * 8 + 8]);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let _ = verify_attestation_trailer(&hasher, LOG_ROUNDS, root, N_QUERIES, &data[32..], &data[..8]);
});
