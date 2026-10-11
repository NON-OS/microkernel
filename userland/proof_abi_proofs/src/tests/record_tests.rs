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

use nonos_proof_abi::{decode_record, RECORD_BYTES};

use crate::fixture::family;

#[test]
fn a_record_reads_back_as_written() {
    let r = family().record;
    let b = r.encode();
    assert_eq!(b.len(), RECORD_BYTES);
    assert_eq!((&b[..4], &b[6..8]), (&b"NFSR"[..], &[1, 0][..]));
    assert_eq!(decode_record(&b), Some(r));
}

#[test]
fn every_reserved_byte_and_unknown_flag_is_refused() {
    let b = family().record.encode();
    for at in (49..56).chain(172..176).chain(208..240) {
        let mut g = b;
        g[at] = 1;
        assert_eq!(decode_record(&g), None, "reserved byte {at}");
    }
    let mut g = b;
    g[6] = 8;
    assert_eq!(decode_record(&g), None, "flag bit 3, after overflow");
}

#[test]
fn another_magic_version_or_length_is_refused() {
    let b = family().record.encode();
    for at in [0, 4, 5] {
        let mut g = b;
        g[at] ^= 0x20;
        assert_eq!(decode_record(&g), None, "byte {at}");
    }
    assert_eq!(decode_record(&b[..239]), None);
    assert_eq!(decode_record(&[&b[..], &[0]].concat()), None);
}

#[test]
fn a_non_canonical_commitment_word_is_refused() {
    let mut b = family().record.encode();
    b[104..112].copy_from_slice(&u64::MAX.to_le_bytes());
    assert_eq!(decode_record(&b), None);
}
