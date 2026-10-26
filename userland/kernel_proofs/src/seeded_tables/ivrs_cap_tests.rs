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
//! An IVHD range END below its START names no requester id, so it takes no
//! span: with any number of them first, the 64 real spans after them are
//! all kept (ivhd_kind.rs, the IVRS sweep's finding).

use super::gen::{sdt, seal};
use crate::proofs::proofs_1::amd_ivhd::ivhd_scope::{spans, Span, MAX_SPANS};

/// An IVRS of one IVHD 10h block holding `entries`.
fn ivrs(entries: &[[u8; 4]]) -> Vec<u8> {
    let length = (24 + 4 * entries.len()) as u16;
    let mut block = [vec![0x10, 0], length.to_le_bytes().to_vec(), vec![0; 4]].concat();
    block.extend((1u64 << 20).to_le_bytes().iter().chain(&[0; 8]));
    block.extend(entries.iter().flatten());
    let mut t = sdt(b"IVRS", 2, &[vec![0; 12], block].concat());
    seal(&mut t);
    t
}

#[test]
fn inverted_ranges_take_no_span_and_every_real_one_after_them_is_kept() {
    for inverted in [1usize, 7, MAX_SPANS] {
        let mut entries = Vec::new();
        for _ in 0..inverted {
            entries.extend([[0x03, 0x50, 0x00, 0], [0x04, 0x10, 0x00, 0]]);
        }
        entries.extend((0..MAX_SPANS as u16).map(|id| {
            let [lo, hi] = (0x100 + id).to_le_bytes();
            [0x02, lo, hi, 0]
        }));
        let mut out = [Span { first: 0, last: 0, named: false }; MAX_SPANS];
        let n = spans(&ivrs(&entries), &mut out);
        assert_eq!(n, MAX_SPANS, "{inverted} inverted first");
        assert!(out
            .iter()
            .enumerate()
            .all(|(i, s)| s.first == 0x100 + i as u16 && s.first == s.last));
    }
}
