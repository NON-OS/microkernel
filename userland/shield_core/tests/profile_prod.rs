// Tests assert by panicking and print measurements, so those lints are off here only.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::integer_division)]
#![allow(clippy::arithmetic_side_effects)]

//! Where a production proof spends its time and memory, phase by phase, for the pinned 37-limb
//! transfer in `PROD_VECTORS`. A first run proves from nothing and writes the periodic cache to
//! `PROD_CACHE`, and the next proves from it, one run per process so each peak is its own.

use nox_prover::api::{prove_with, Options, Phase};
use std::sync::Mutex;
use std::time::Instant;

fn read(name: &str) -> Vec<u8> {
    let dir = std::env::var("PROD_VECTORS").expect("PROD_VECTORS");
    std::fs::read(format!("{dir}/transfer-eth/{name}")).expect(name)
}

fn text(name: &str) -> String {
    String::from_utf8(read(name)).expect(name)
}

fn peak_mb() -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let line = status.lines().find(|l| l.starts_with("VmHWM:")).unwrap_or("VmHWM: 0 kB");
    line.split_whitespace().nth(1).and_then(|k| k.parse::<u64>().ok()).unwrap_or(0) / 1024
}

#[test]
#[ignore = "profile: needs the pinned vectors, one run per process"]
fn the_production_proof_by_phase() {
    let hex = text("entropy.hex");
    let e: Vec<u8> = (0..hex.trim().len() / 2)
        .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).expect("hex"))
        .collect();
    let (request, seed, path) = (text("request.json"), text("seed.json"), env("PROD_CACHE"));
    let start = Instant::now();
    let Ok(cache) = std::fs::read(&path) else {
        let (_, cache) = nox_prover::prove_keeping_cache(&request, &seed, &e).expect("a proof");
        std::fs::write(&path, &cache).expect("the cache");
        let s = start.elapsed().as_secs_f64();
        return println!("from nothing {s:.2} s, peak {} MB, cache kept", peak_mb());
    };
    let marks: Mutex<Vec<(Phase, Instant)>> = Mutex::new(Vec::new());
    let mark =
        |p: Phase, _: f32| marks.lock().map(|mut m| m.push((p, Instant::now()))).unwrap_or(());
    let opts = Options { cache: Some(&cache), progress: Some(&mark), cancel: None };
    let (proof, _) = prove_with(&request, &seed, &e, &opts).expect("proves");
    let total = start.elapsed().as_secs_f64();
    assert_eq!(proof.bytes, read("proof.bin"), "the pinned proof, byte for byte");
    let mut last = start;
    for (phase, at) in marks.into_inner().unwrap_or_default() {
        let s = at.duration_since(last).as_secs_f64();
        println!("{phase:?} | {s:.2} s | {:.1}%", 100.0 * s / total);
        last = at;
    }
    let threads = std::env::var("RAYON_NUM_THREADS").unwrap_or_else(|_| "all".into());
    println!("from the cache {total:.2} s, peak {} MB, threads {threads}", peak_mb());
}

fn env(name: &str) -> String {
    std::env::var(name).expect(name)
}
