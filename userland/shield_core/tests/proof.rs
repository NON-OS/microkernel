// Tests assert by panicking and print measurements, so those lints are off here only.
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use nox_shield_core::bench::bench_launch;
use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::keys::Account;
use nox_shield_core::prover::Cancel;

fn account() -> Account {
    let phrase = generate_phrase().expect("entropy");
    Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account")
}

/// Gates 1 and 2: a real transfer proved by the v2 prover at shape A and verified by it, in the
/// shared form the pool reads. It takes minutes,
/// so run it with `cargo test --release -- --ignored a_transfer_proves`.
#[test]
#[ignore = "proves at the launch point, which takes minutes"]
fn a_transfer_proves_and_verifies_on_this_machine() {
    // NOX_PROOF_OUT keeps the proof for the live verifier check in CI.
    let dir = std::env::var("NOX_PROOF_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| {
        std::env::temp_dir().join(format!("nox-launch-bench-{}", std::process::id()))
    });
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let report =
        bench_launch(&account(), &dir, &Cancel::new()).expect("a proof at the launch point");
    assert!(report.verified);
    assert!(report.launch_point, "the proof's parameter id is {}", report.params_id);
    assert!(
        (80_000..=110_000).contains(&report.proof_bytes),
        "a v2 proof, {} bytes",
        report.proof_bytes
    );
    println!(
        "proved {} ms, {} bytes, params {}",
        report.proved_ms, report.proof_bytes, report.params_id
    );
    if std::env::var("NOX_PROOF_OUT").is_err() {
        std::fs::remove_dir_all(&dir).ok();
    }
}
