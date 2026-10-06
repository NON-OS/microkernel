//! The profile proves the vector it names, and its times account for the whole proof.

// A test asserts by panicking and builds instants by hand, so those lints are off here only.
#![allow(clippy::expect_used, clippy::arithmetic_side_effects, clippy::panic)]

use super::marks::split;
use super::report::ProofPhase::{self, *};
use super::{pool, vector};
use sha3::{Digest, Keccak256};
use std::time::{Duration, Instant};

const ORDER: [ProofPhase; 11] = [
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

fn keccak(text: &str) -> String {
    Keccak256::digest(text.as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn the_bundled_inputs_are_the_pinned_vector() {
    let request = "2c8eb970e214d5f0e2fabc8943d28c672b4c0662418b9f076a9601fcc19d1240";
    let seed = "c59477a5deefe9e5fc967c74abdcef7c8fba7a4ba785e08498e5fadd0d14a9f8";
    let entropy = "8cfd7d3797d181fe6000d6becade942cdda4cf72213a67755855edf31ea750d7";
    assert_eq!(keccak(vector::REQUEST), request);
    assert_eq!(keccak(vector::SEED), seed);
    assert_eq!(keccak(vector::ENTROPY), entropy);
    let bytes = vector::entropy().expect("512 bytes of hex");
    let text: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(text, vector::ENTROPY.trim());
}

#[test]
fn the_phases_and_the_tail_add_up_to_the_total() {
    let start = Instant::now();
    let at = |ms: u64| start + Duration::from_micros(ms * 1_000 + 7);
    let marks: Vec<_> = ORDER.iter().zip(1..).map(|(&p, i)| (p, at(i * i))).collect();
    let (phases, after, total) = split(start, &marks, at(130)).expect("ordered marks");
    let sum: u64 = phases.iter().map(|p| p.micros).sum();
    assert_eq!(sum + after, total);
    assert_eq!(phases.iter().map(|p| p.phase).collect::<Vec<_>>(), ORDER);
    let late = [(Request, at(5)), (Blinding, at(4))];
    assert!(split(start, &late, at(9)).is_none(), "a mark before the one ahead of it");
    assert!(split(at(1), &[(Request, start)], at(9)).is_none(), "a mark before the start");
}

#[test]
fn a_set_thread_count_is_the_pool_the_prover_gets() {
    assert_eq!(pool::on(2, rayon::current_num_threads).expect("a pool"), (2, 2));
    let all = u32::try_from(rayon::current_num_threads()).expect("a count");
    assert_eq!(pool::on(0, || ()).expect("the shared pool").1, all);
}
