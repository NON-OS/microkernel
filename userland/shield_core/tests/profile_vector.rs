// Tests assert by panicking and print measurements, so those lints are off here only.
#![allow(clippy::expect_used, clippy::arithmetic_side_effects)]

//! The phase profile proves the pinned `transfer-eth` vector to its `proof.json`, and its times
//! account for the whole proof. `PROFILE_THREADS` picks the threads, 0 or unset for every core,
//! and a first run builds the periodic cache.

use nox_shield_core::bench::{profile, ProofPhase::*};
use nox_shield_core::prover::Cancel;

#[test]
#[ignore = "proves the vector: about a minute and a half on a server, longer on the first run"]
fn the_vector_proves_and_its_times_add_up() {
    let order = [
        Request,
        Blinding,
        Region,
        Products,
        Periodic,
        Composition,
        CompositionTree,
        Deep,
        Fri,
        Queries,
        Verified,
    ];
    let threads = std::env::var("PROFILE_THREADS").map_or(0, |t| t.parse().expect("a count"));
    let dir = std::env::temp_dir().join("nox-profile-test");
    std::fs::create_dir_all(&dir).expect("a folder");
    let run = profile(&dir, threads, &Cancel::new()).expect("a profile");
    assert!(run.matched, "the proof is not the pinned proof.json");
    assert_eq!(run.phases.iter().map(|p| p.phase).collect::<Vec<_>>(), order);
    let sum: u64 = run.phases.iter().map(|p| p.micros).sum();
    assert!((sum + run.after_verified_micros).abs_diff(run.total_micros) <= 1_000);
    println!("{run:#?}\nphases {sum} us, after Verified {} us", run.after_verified_micros);
}
