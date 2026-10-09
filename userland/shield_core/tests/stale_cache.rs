// Tests assert by panicking and print measurements, so those lints are off here only.
#![allow(clippy::expect_used, clippy::panic)]

use nox_shield_core::bench::bench_launch;
use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::keys::Account;
use nox_shield_core::prover::launch::cache;
use nox_shield_core::prover::Cancel;

/// A phone that proved before v2 holds the launch cache as `periodic.top`, and a damaged file can
/// sit under the name of this circuit. The old file must go, the damaged one must be rebuilt, and
/// the proof must complete either way.
#[test]
#[ignore = "proves at shape A, which takes minutes"]
fn a_stale_periodic_cache_is_rebuilt_and_the_proof_completes() {
    let phrase = generate_phrase().expect("entropy");
    let account = Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account");
    let dir = std::env::temp_dir().join(format!("nox-stale-cache-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let old = dir.join("periodic.top");
    let own = cache::path(&dir);
    std::fs::write(&old, b"a cache from the launch circuit").expect("the old cache");
    std::fs::write(&own, b"NXTT a damaged cache").expect("the damaged cache");
    let report = bench_launch(&account, &dir, &Cancel::new()).expect("a proof despite the cache");
    assert!(report.verified);
    assert!(!old.exists(), "the launch cache was kept");
    let rebuilt = std::fs::read(&own).expect("a cache in its place");
    assert!(!rebuilt.starts_with(b"NXTT a damaged"), "the damaged cache was kept");
    println!("proved {} ms, cache {} bytes", report.proved_ms, rebuilt.len());
    std::fs::remove_dir_all(&dir).ok();
}
