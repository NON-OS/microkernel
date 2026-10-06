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
 * Hostile headers: every header byte flipped (the magic, the kind and the path
 * length; the parameter id lives in the proof's own header), lengths that point
 * past the end or wrap, and an inner path that is not a v3 trailer.
 */

use super::fixture::policy;
use crate::leaf::Kind;
use crate::v4::{encode_v4, parse_v4, MAX_PROOF_V4};

fn good() -> (Vec<u8>, usize) {
    let (_, tree, _, _) = policy();
    let path = tree.trailer(0).expect("trailer");
    let t = encode_v4(Kind::Capsule, &path, &[1u8; 64]).expect("encode");
    (t, path.len())
}

#[test]
fn a_changed_magic_kind_or_length_byte_is_refused() {
    let (t, _) = good();
    for i in 0..13 {
        let mut bad = t.clone();
        bad[i] ^= 0x01;
        assert!(parse_v4(&bad, Kind::Capsule, MAX_PROOF_V4).is_none(), "byte {i}");
    }
}

#[test]
fn lengths_past_the_end_or_at_the_word_limit_are_refused() {
    let (t, path_len) = good();
    for v in [u32::MAX, u32::MAX - 12, (t.len() as u32) + 1] {
        let mut bad = t.clone();
        bad[9..13].copy_from_slice(&v.to_le_bytes());
        assert!(parse_v4(&bad, Kind::Capsule, MAX_PROOF_V4).is_none(), "path length {v}");
        let mut bad = t.clone();
        bad[13 + path_len..17 + path_len].copy_from_slice(&v.to_le_bytes());
        assert!(parse_v4(&bad, Kind::Capsule, MAX_PROOF_V4).is_none(), "proof length {v}");
    }
}

#[test]
fn the_writer_refuses_what_the_reader_would() {
    let (_, tree, _, _) = policy();
    let path = tree.trailer(0).expect("trailer");
    assert!(encode_v4(Kind::Pad, &path, &[1]).is_none());
    assert!(encode_v4(Kind::Capsule, b"not a path", &[1]).is_none());
    assert!(encode_v4(Kind::Capsule, &path, &[]).is_none());
    assert!(encode_v4(Kind::Capsule, &path, &vec![0u8; MAX_PROOF_V4 + 1]).is_none());
}
