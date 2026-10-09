// Tests assert by panicking and print measurements, so those lints are off here only.
#![allow(clippy::expect_used, clippy::panic)]

//! A spend proved from the periodic cache reports the prover's phases on its cancel token, in
//! order, ending at Verified with the whole proof done. `PERIODIC_CACHE` names the cache file,
//! the one `nix build .#periodic-cache` makes.

use nox_prover::Phase;
use nox_shield_core::bench::bench_launch;
use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::keys::Account;
use nox_shield_core::prover::launch::cache;
use nox_shield_core::prover::Cancel;

#[test]
#[ignore = "proves at shape A, which takes minutes; needs PERIODIC_CACHE"]
fn a_spend_reports_how_far_its_proof_has_come() {
    let top = std::fs::read(std::env::var("PERIODIC_CACHE").expect("PERIODIC_CACHE"))
        .expect("the periodic cache");
    let phrase = generate_phrase().expect("entropy");
    let account = Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account");
    let dir = std::env::temp_dir().join(format!("nox-progress-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    cache::keep(&dir, &top);
    let cancel = Cancel::new();
    assert_eq!(cancel.progress(), None, "no phase before the proof");
    let report = bench_launch(&account, &dir, &cancel).expect("a proof");
    assert!(report.verified);
    let (phase, fraction) = cancel.progress().expect("the prover reported its phases");
    assert_eq!(phase, Phase::Verified);
    assert!((fraction - 1.0).abs() < f32::EPSILON, "the last report is the whole proof");
    println!("proved {} ms, last report {phase:?} at {fraction}", report.proved_ms);
    std::fs::remove_dir_all(&dir).ok();
}
