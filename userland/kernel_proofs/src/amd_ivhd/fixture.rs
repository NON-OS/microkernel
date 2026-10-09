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

use super::ivhd_scope::{spans, Span, MAX_SPANS};

/// An IVRS of one IVHD block of `kind` holding `entries`.
pub(super) fn ivrs(kind: u8, entries: &[u8]) -> Vec<u8> {
    let header = if kind == 0x10 { 24 } else { 40 };
    let mut t = vec![0u8; 48];
    let mut block = vec![0u8; header];
    block[0] = kind;
    block[8..16].copy_from_slice(&0xfd20_0000u64.to_le_bytes());
    block.extend_from_slice(entries);
    let len = block.len() as u16;
    block[2..4].copy_from_slice(&len.to_le_bytes());
    t.extend_from_slice(&block);
    t
}

pub(super) fn read(table: &[u8]) -> Vec<Span> {
    let mut out = [Span { first: 0, last: 0, named: false }; MAX_SPANS];
    let n = spans(table, &mut out);
    out[..n].to_vec()
}

pub(super) const fn span(first: u16, last: u16, named: bool) -> Span {
    Span { first, last, named }
}
