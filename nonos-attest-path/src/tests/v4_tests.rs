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

/*
 * The v4 container around a real enrolled path. Each refusal starts from a
 * trailer that reads back, so it is never an artefact of a bad fixture.
 */

use super::fixture::{capsule_ctx, policy, DEPTH, EPOCH};
use crate::leaf::Kind;
use crate::v4::{encode_v4, parse_v4, MAX_PROOF_V4};
use crate::verify::verify;

const PROOF: &[u8] = &[0xab; 1000];

fn good() -> ([u8; 32], Vec<u8>, Vec<u8>) {
    let (root, tree, cap, _) = policy();
    let path = tree.trailer(0).expect("trailer");
    let t = encode_v4(Kind::Capsule, &path, PROOF).expect("encode");
    (root, t, capsule_ctx(&cap, 0x7, EPOCH))
}

#[test]
fn a_trailer_reads_back_and_its_path_still_verifies() {
    let (root, t, ctx) = good();
    let v = parse_v4(&t, Kind::Capsule, MAX_PROOF_V4).expect("parse");
    assert_eq!((v.kind, v.proof), (Kind::Capsule, PROOF));
    assert!(verify(&root, DEPTH, Kind::Capsule, &ctx, v.path));
}

#[test]
fn another_kind_than_the_gate_expects_is_refused() {
    let (_, t, _) = good();
    assert!(parse_v4(&t, Kind::Kernel, MAX_PROOF_V4).is_none());
    assert!(parse_v4(&t, Kind::Bootloader, MAX_PROOF_V4).is_none());
    assert!(parse_v4(&t, Kind::Pad, MAX_PROOF_V4).is_none());
}

#[test]
fn every_truncation_and_any_trailing_byte_is_refused() {
    let (_, t, _) = good();
    for n in 0..t.len() {
        assert!(parse_v4(&t[..n], Kind::Capsule, MAX_PROOF_V4).is_none(), "length {n}");
    }
    let mut long = t.clone();
    long.push(0);
    assert!(parse_v4(&long, Kind::Capsule, MAX_PROOF_V4).is_none());
}

#[test]
fn a_proof_over_the_callers_bound_is_refused() {
    let (_, t, _) = good();
    assert!(parse_v4(&t, Kind::Capsule, PROOF.len()).is_some());
    assert!(parse_v4(&t, Kind::Capsule, PROOF.len() - 1).is_none());
    assert!(parse_v4(&t, Kind::Capsule, 0).is_none());
    assert!(parse_v4(&t, Kind::Capsule, MAX_PROOF_V4 + 1).is_none());
}
