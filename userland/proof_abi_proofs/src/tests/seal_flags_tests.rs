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

//! A2b's record and container rules: an overflowed record reads but never
//! verifies, and a kind is one byte's worth, as the anchor's uint8 is.

use nonos_proof_abi::{decode_record, parse, tag, verify, Refusal, FLAG_FINAL, FLAG_OVERFLOW};

use super::container_tests::sample;
use crate::fixture::{expected, honest};

#[test]
fn an_overflowed_record_reads_and_is_refused() {
    let (mut f, mut parts, pinned) = honest();
    f.record.flags = FLAG_FINAL | FLAG_OVERFLOW;
    let raw = f.record.encode();
    assert_eq!(decode_record(&raw).map(|r| r.flags), Some(FLAG_FINAL | FLAG_OVERFLOW));
    let mut other = raw;
    other[6] |= 8;
    assert!(decode_record(&other).is_none(), "bit 3 means nothing");
    parts.set(tag::RECORD, raw.to_vec());
    let pins = pinned.trust();
    assert_eq!(verify(&parts.bytes(), &expected(&f.policy, &pins)), Err(Refusal::Supervisor));
}

#[test]
fn a_kind_past_one_byte_is_refused() {
    let mut b = sample();
    assert!(parse(&b).is_some());
    b[6..8].copy_from_slice(&257u16.to_le_bytes()); // 1 as a uint8
    assert!(parse(&b).is_none());
}
