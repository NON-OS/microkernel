// The aggregate verifier's parse and verdict on arbitrary bytes. The header
// and entry framing carry counts and offsets an attacker chooses; the parse
// must refuse them without ever reading past the blob.
#![no_main]
use libfuzzer_sys::fuzz_target;
use nonos_stark::air::{verify_aggregate, Poseidon, RATE};
use nonos_stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use nonos_stark::field::Fp;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }
    let mut root = [Fp::ZERO; RATE];
    for (i, lane) in root.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&data[i * 8..i * 8 + 8]);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let _ = verify_aggregate(
        &hasher,
        LOG_ROUNDS,
        root,
        &data[32..],
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
});
