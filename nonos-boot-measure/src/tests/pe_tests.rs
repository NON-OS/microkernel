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

use sha2::{Digest, Sha256};

use super::pe_build::{fields, image, put, sign, HEADERS, OPT};
use crate::authenticode::digest;

/// The specification's composition, written out: headers without the checksum
/// and the certificate entry, then the sections in file order.
fn expected(f: &[u8], plus: bool, in_file_order: &[(usize, usize)]) -> [u8; 32] {
    let (cs, ce) = fields(plus);
    let mut h = Sha256::new();
    h.update(&f[..cs]);
    h.update(&f[cs + 4..ce]);
    h.update(&f[ce + 8..HEADERS]);
    in_file_order.iter().for_each(|&(at, n)| h.update(&f[at..at + n]));
    /* Past the headers and the sections' bytes, the rest to the end, gaps too. */
    let sum = HEADERS + in_file_order.iter().map(|s| s.1).sum::<usize>();
    h.update(&f[sum.min(f.len())..]);
    h.finalize().into()
}

#[test]
fn sections_are_hashed_in_file_order_for_both_layouts() {
    for plus in [true, false] {
        let f = image(plus, &[(0x400, 0x80, 0xBB), (0x200, 0x100, 0xAA), (0x300, 0, 0)]);
        assert_eq!(digest(&f), Ok(expected(&f, plus, &[(0x200, 0x100), (0x400, 0x80)])));
    }
}

#[test]
fn a_signature_the_checksum_and_the_certificate_entry_leave_the_digest() {
    let f = image(true, &[(0x200, 0x100, 0xAA)]);
    let honest = digest(&f).expect("digests");
    let mut signed = f.clone();
    sign(&mut signed, true, &[0x30; 1000]);
    put(&mut signed, OPT + 64, &0x1234u32.to_le_bytes());
    assert_eq!(digest(&signed), Ok(honest));
}

#[test]
fn any_hashed_byte_moves_the_digest_and_data_past_the_sections_is_hashed() {
    let f = image(true, &[(0x200, 0x100, 0xAA)]);
    let honest = digest(&f).expect("digests");
    let (cs, ce) = fields(true);
    for i in (0..f.len()).filter(|i| !(cs..cs + 4).contains(i) && !(ce..ce + 8).contains(i)) {
        let mut m = f.clone();
        m[i] ^= 0x80;
        assert_ne!(digest(&m).ok(), Some(honest), "byte {i}");
    }
    let mut tail = f.clone();
    tail.extend([1, 2, 3]);
    assert_ne!(digest(&tail).ok(), Some(honest));
}
