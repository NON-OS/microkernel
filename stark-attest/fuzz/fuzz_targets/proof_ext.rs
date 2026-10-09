// The proof deserializer alone, past the trailer framing: length fields,
// nested paths, field element canonicality, all attacker controlled.
#![no_main]
use libfuzzer_sys::fuzz_target;
use nonos_stark::air::deserialize_proof_ext;

fuzz_target!(|data: &[u8]| {
    let _ = deserialize_proof_ext(data);
});
