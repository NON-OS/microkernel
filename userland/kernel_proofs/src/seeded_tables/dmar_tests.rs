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
//! DMAR in the seeded sweep: headed tables whose structures and scopes lie,
//! through the kernel's header check and its pure walk and parsers
//! (dmar_scope's mounts). Every structure handed on lies whole inside the
//! declared length, back to back from the first; the walk is cut short only
//! where non-padding bytes are left; what each parser keeps holds its rules.

use std::cell::Cell;

use super::dmar_gen::dmar_body;
use super::dmar_read::read;
use super::gen::{damage, sdt, Rng};
use super::sweep::sweep;
use crate::dmar_scope::dmar_walk::FIRST_STRUCTURE;

#[test]
fn dmar_structures_stay_inside_the_table_and_each_parser_keeps_its_rules() {
    let make = |r: &mut Rng| damage(sdt(b"DMAR", 1, &dmar_body(r)), r);
    let kept = Cell::new(0u8);
    sweep(make, read, |s, t| {
        kept.set(kept.get() | s.kept);
        let n = u32::from_le_bytes(t[4..8].try_into().unwrap()) as usize;
        let starts = s.at.iter().map(|a| a.0);
        let ends = [FIRST_STRUCTURE].into_iter().chain(s.at.iter().map(|a| a.1));
        let chained = starts.zip(ends).all(|(start, end)| start == end);
        let declared = |a: usize| u16::from_le_bytes([t[a + 2], t[a + 3]]) as usize;
        let whole = s.at.iter().all(|&(a, b)| b >= a + 4 && b <= n && b - a == declared(a));
        let rest = &t[s.at.last().map_or(FIRST_STRUCTURE, |a| a.1).min(n)..n];
        let ended = rest.len() < 4 || rest.iter().all(|&b| b == 0);
        chained && whole && s.sound && s.cut != ended
    });
    assert_eq!(kept.get(), 0b111, "a DRHD, an RMRR and an ANDD each kept somewhere");
}
