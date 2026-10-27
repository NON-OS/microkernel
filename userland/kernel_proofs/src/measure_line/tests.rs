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

use super::measure_line::{line, LEN};

const FINISH: &str = include_str!("../../../../src/process/foreign/seal/finish.rs");
const HIST: &str = include_str!("../../../../src/process/foreign/seal/hist.rs");
const PROVER: &str = include_str!("../../../capsule_prove/src/market/buckets.rs");

fn hist(ok: &[usize], refused: &[usize]) -> Vec<u32> {
    let mut h = vec![0u32; 2 * 513];
    ok.iter().for_each(|&b| h[2 * b] = 3);
    refused.iter().for_each(|&b| h[2 * b + 1] = 5);
    h
}

fn set_of(said: &[u8]) -> Vec<u8> {
    let text = core::str::from_utf8(said).expect("ascii");
    let (_, hex) = text.split_once(" ok=").expect("the set");
    (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn the_line_names_the_app_and_sets_a_bit_for_each_bucket_that_went_through() {
    let id: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_mul(17));
    let said = line(&id, &hist(&[0, 9, 231, 511, 512], &[1, 60]));
    assert_eq!(said.len(), LEN);
    assert!(said.starts_with(b"[MEASURE] app=0011223344556677 ok="));
    let set = set_of(&said);
    assert_eq!(set.len(), 65);
    let on: Vec<usize> = (0..520).filter(|&b| set[b / 8] & (1 << (b % 8)) != 0).collect();
    assert_eq!(on, [0, 9, 231, 511, 512], "a refused call counted, or a bucket lost");
}

#[test]
fn the_set_is_the_provers_shape_over_the_kernels_buckets() {
    assert!(HIST.contains("pub(crate) const BUCKETS: usize = 513;"));
    assert!(PROVER.contains("pub const LINE_LEN: usize = 27 + 130;"));
    assert!(PROVER.contains("filter(|&b| hist[2 * b] > 0)"), "the prover reads side 0");
    let quiet = line(&[0; 32], &hist(&[], &[0, 512]));
    assert!(set_of(&quiet).iter().all(|&b| b == 0), "a refusal set a bit");
}

#[test]
fn only_a_build_with_capsule_serial_output_says_it() {
    let at = FINISH.find("measure_line::line").expect("the hook");
    let gate = FINISH[..at].rfind("if cfg!(feature = \"capsule-serial-debug\")").expect("its gate");
    assert!(!FINISH[gate..at].contains('}'), "the line is said outside its gate");
}
