// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Test 5, on the machine: the four pinned production wallet vectors proved
//! by the nox_prover nonos.shield links, built as nonos.shield is built
//! (the std layer, its heap, a rayon worker per online core), each proof
//! and its format 7 form held to the pinned bytes. One serial line per
//! check, PASS or FAIL, then a DONE line with the count. Development
//! images only.
//!
//! The first vector proves from nothing, as a wallet with no cache would,
//! and the tree it builds must be the cache nonos.shield ships. The other
//! three prove from the shipped cache, as nonos.shield does.

mod heap;
mod machine;
mod pool;
mod vectors;

use std::time::Instant;

use nox_prover::{Options, Phase};

static SHIPPED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/periodic.top"));

const TAG: &str = "SHIELD-VECTORS";

fn line(ok: bool, what: &str) -> bool {
    println!("{TAG} {} {what}", if ok { "PASS" } else { "FAIL" });
    ok
}

/// The periodic root check on the shipped file: taken whole, refused with
/// one bit of its root changed.
fn shipped_cache() -> bool {
    if SHIPPED.is_empty() {
        println!("{TAG} cache: none shipped in this build; the first proof builds it");
        return true;
    }
    let taken = nox_prover::load_cache(SHIPPED).is_ok();
    let mut touched = SHIPPED.to_vec();
    if let Some(last) = touched.last_mut() {
        *last ^= 1;
    }
    let refused = nox_prover::load_cache(&touched).is_err();
    line(taken, &format!("cache: the shipped {} bytes carry PERIODIC_ROOT", SHIPPED.len()))
        & line(refused, "cache: the same file with one bit of its root changed is refused")
}

/// Prove `v`, from `cache` when held, and hold the result to the pinned
/// bytes. Returns the cache the proof had or built.
fn prove(v: &vectors::Vector, cache: Option<Vec<u8>>, passed: &mut u32) -> Option<Vec<u8>> {
    let Some(entropy) = vectors::entropy(v) else {
        line(false, &format!("{}: the entropy file is not {} bytes", v.name, nox_prover::ENTROPY_BYTES));
        return cache;
    };
    let free_before = machine::mem_free_mib();
    heap::reset_peak();
    let watch = machine::Watch::start();
    let start = Instant::now();
    let report = |p: Phase, f: f32| {
        println!("{TAG} {}: {p:?} done, {:.0}% at {} ms", v.name, f * 100.0, start.elapsed().as_millis());
    };
    let proved = match &cache {
        Some(top) => {
            let opts = Options { cache: Some(top), progress: Some(&report), cancel: None };
            nox_prover::prove_with(v.request, v.seed, &entropy, &opts).map(|(p, _)| (p, None)).map_err(|e| e.to_string())
        }
        None => nox_prover::prove_keeping_cache(v.request, v.seed, &entropy).map(|(p, c)| (p, Some(c))),
    };
    let ms = start.elapsed().as_millis();
    let least_free = watch.stop();
    let (proof, built) = match proved {
        Ok(done) => done,
        Err(why) => {
            line(false, &format!("{}: refused after {ms} ms: {why}", v.name));
            return cache;
        }
    };
    let from_nothing = cache.is_none();
    let from = if from_nothing { "from nothing" } else { "from the periodic cache" };
    println!(
        "{TAG} {}: proved {from} in {ms} ms, heap peak {} MiB, machine free {} MiB before, least {} MiB",
        v.name,
        heap::peak_mib(),
        free_before,
        least_free
    );
    let cache = cache.or(built);
    let Some(top) = cache.as_deref() else {
        line(false, &format!("{}: no periodic cache after the proof", v.name));
        return None;
    };
    if from_nothing && !SHIPPED.is_empty() {
        line(top == SHIPPED, &format!("{}: the tree built here is the shipped cache, byte for byte", v.name));
    }
    let json = nox_prover::to_json(&proof);
    let shared = nox_prover::to_shared(&proof.bytes, &proof.publics, top);
    let same_json = json.as_bytes() == v.proof_json;
    let same_shared = shared.as_deref().ok() == Some(v.format7);
    let verifies = shared.as_ref().is_ok_and(|s| nox_prover::verify_shared(s, &proof.publics, top).is_ok());
    let ok = line(same_json, &format!("{}: proof.json byte for byte ({} proof bytes)", v.name, proof.bytes.len()))
        & line(same_shared, &format!("{}: proof-format7.bin byte for byte ({} bytes)", v.name, v.format7.len()))
        & line(verifies, &format!("{}: the format 7 form verifies", v.name));
    if ok {
        *passed += 1;
    }
    cache
}

fn main() {
    pool::start();
    println!(
        "{TAG} start: {} cores online, {} prover workers, {} MiB memory, {} MiB free",
        machine::cpus_online(),
        pool::workers(),
        machine::mem_total_mib(),
        machine::mem_free_mib()
    );
    if pool::workers() == 0 {
        line(false, "no thread could be started for the prover");
        println!("{TAG} DONE 0 of {} vectors", vectors::ALL.len());
        return;
    }
    let cache_ok = shipped_cache();
    let mut passed = 0u32;
    let mut cache: Option<Vec<u8>> = None;
    for (i, v) in vectors::ALL.iter().enumerate() {
        // The first proves from nothing; the rest from the shipped cache,
        // or from the tree the first built when this build ships none.
        if i == 1 && !SHIPPED.is_empty() {
            cache = Some(SHIPPED.to_vec());
        }
        cache = prove(v, cache, &mut passed);
    }
    println!(
        "{TAG} DONE {passed} of {} vectors byte for byte{}",
        vectors::ALL.len(),
        if cache_ok { "" } else { ", shipped cache FAILED" }
    );
}
