//! One real transfer proved on this device through the v2 prover, in the shared form the pool
//! verifies. The proof and its public words are exported for the verifier, and its header's
//! parameter id is checked against shape A so a rehearsal circuit cannot be reported as the real one.

#[path = "launch_shape.rs"]
mod shape;

use super::memory::peak_kib;
use super::report::BenchReport;
use crate::error::WalletError;
use crate::keys::Account;
use crate::prover::launch::{fixture::transfer, prove::prove_spend};
use crate::prover::Cancel;
use shape::{LAUNCH_SHAPE, V2_PARAMS_ID};
use std::path::Path;
use std::time::Instant;

/// Prove the fixture transfer and report its cost. The proof is written under `dir`.
pub fn bench_launch(
    account: &Account,
    dir: &Path,
    cancel: &Cancel,
) -> Result<BenchReport, WalletError> {
    let t0 = Instant::now();
    let (request, a, b) = transfer(account);
    let secret = account.sk().map(|f| f.value());
    // The periodic cache from the first proof, under the root it must carry.
    let cache = crate::prover::launch::cache::read(dir);
    let built_ms = millis(t0);
    let t1 = Instant::now();
    let proof = prove_spend(&request, &secret, [&a, &b], cache.as_deref(), cancel)?;
    let proved_ms = millis(t1);
    if let Some(built) = &proof.new_cache {
        crate::prover::launch::cache::keep(dir, built);
    }
    let params_id = proof.bytes.get(8..40).map(hex).unwrap_or_default();
    let publics: Vec<String> = proof.publics.iter().map(u64::to_string).collect();
    // Its own folder, so a platform share can expose it and nothing beside it.
    let out = dir.join("export");
    let bin = out.join("launch-proof.bin");
    let json = out.join("launch-proof.publics.json");
    let written = std::fs::create_dir_all(&out).is_ok()
        && std::fs::write(&bin, &proof.bytes).is_ok()
        && std::fs::write(
            &json,
            format!("{{\"n_publics\": 36, \"publics\": [{}]}}\n", publics.join(", ")),
        )
        .is_ok();
    Ok(BenchReport {
        built_ms,
        proved_ms,
        // The prover verifies before it returns, inside the proving time.
        verified_ms: 0,
        verified: true,
        proof_bytes: proof.bytes.len() as u64,
        peak_kib: peak_kib(),
        shape: LAUNCH_SHAPE,
        launch_point: params_id == V2_PARAMS_ID,
        params_id,
        proof_path: written.then(|| bin.to_string_lossy().into_owned()),
    })
}

fn millis(since: Instant) -> u64 {
    u64::try_from(since.elapsed().as_millis()).unwrap_or(u64::MAX)
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
