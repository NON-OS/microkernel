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
//! One IVRS read as the kernel reads it (ivrs.rs): admitted by its header,
//! then every pure walk over the same bytes, each into room of its cap.
//! The tables made here stay under ivrs.rs's 64 KiB cut, so the bytes the
//! kernel walks are the declared length.

use super::admit::admitted;
use crate::proofs::proofs_1::amd_ivhd::ivhd_alias::{aliases, Alias, MAX_ALIASES};
use crate::proofs::proofs_1::amd_ivhd::ivhd_scope::{spans, Span, MAX_SPANS};
use crate::proofs::proofs_1::amd_ivhd::ivhd_special::{specials, MAX_SPECIALS};
use crate::proofs::proofs_1::amd_ivhd::ivmd::{reserved, Reserved, MAX_RESERVED};
use crate::proofs::proofs_2::firmware_iommu::ivrs_walk::{walk, MAX_AMD_IOMMUS};

pub struct Seen {
    pub bases: Vec<u64>,
    pub complete: bool,
    pub spans: Vec<Span>,
    pub aliases: Vec<Alias>,
    pub specials: Vec<u16>,
    pub reserved: Vec<Reserved>,
}

pub fn read(t: &[u8]) -> Option<Seen> {
    let t = admitted(t)?;
    let mut found = [0u64; MAX_AMD_IOMMUS];
    let w = walk(t, &mut found);
    let mut s = [Span { first: 0, last: 0, named: false }; MAX_SPANS];
    let ns = spans(t, &mut s);
    let mut a = [Alias { first: 0, last: 0, alias: 0 }; MAX_ALIASES];
    let (na, _) = aliases(t, &mut a);
    let mut sp = [0u16; MAX_SPECIALS];
    let (nsp, _) = specials(t, &mut sp);
    let none = Reserved { first: 0, last: 0, start: 0, end: 0, read: false, write: false };
    let mut r = [none; MAX_RESERVED];
    let nr = reserved(t, &mut r);
    Some(Seen {
        bases: found[..w.found].to_vec(),
        complete: w.complete,
        spans: s[..ns].to_vec(),
        aliases: a[..na].to_vec(),
        specials: sp[..nsp].to_vec(),
        reserved: r[..nr].to_vec(),
    })
}

/// The blocks back to back from offset 48 as the specification lays them
/// out, each `(start, kind, length)`, and whether they end cleanly: fewer
/// than four bytes left, not a block that runs past the table.
pub fn blocks(t: &[u8]) -> (Vec<(usize, u8, usize)>, bool) {
    let (mut out, mut at) = (Vec::new(), 48);
    while at + 4 <= t.len() {
        let length = u16::from_le_bytes([t[at + 2], t[at + 3]]) as usize;
        if length < 4 || at + length > t.len() {
            return (out, false);
        }
        out.push((at, t[at], length));
        at += length;
    }
    (out, true)
}
