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

//! The fixed-layout tables, HPET, FADT and SPCR, and the check a table AML
//! loads passes: every length from none to past the layout, declared
//! lengths that lie, every revision. A field past the declared length is
//! not read.

use super::fadt_decode::decode_fadt;
use super::gen::{damage, sdt, Rng};
use super::hpet_decode::hpet_base;
use super::sweep::{headed, held, sweep};
use crate::arm_acpi::spcr::spcr;
use crate::hardware::broker::acpi::loaded_rule::checked;

#[test]
fn hpet_base_comes_only_from_a_whole_table() {
    let make = |r: &mut Rng| {
        let mut b = r.some_bytes(26);
        if b.len() > 4 && !r.one_in(4) {
            b[4] = 0;
        }
        damage(sdt(b"HPET", 1, &b), r)
    };
    sweep(make, hpet_base, |base, t| {
        held(t) >= 56 && t[40] == 0 && *base != 0 && t[44..52] == base.to_le_bytes()
    });
}

#[test]
fn fadt_reads_nothing_past_its_length() {
    let make = |r: &mut Rng| damage(sdt(b"FACP", 6, &r.some_bytes(260)), r);
    sweep(make, decode_fadt, |f, t| {
        let mut past = t.to_vec();
        past[held(t)..].iter_mut().for_each(|b| *b ^= 0x5A);
        decode_fadt(&past) == Some(*f) && t[4..8] == f.length.to_le_bytes()
    });
}
#[test]
fn spcr_consoles_load_or_are_refused() {
    let make = |r: &mut Rng| {
        let mut body = [r.bytes(44), r.some_bytes(70)].concat();
        if r.one_in(2) {
            body[28..30].copy_from_slice(&[0xFF, 0xFF]);
        }
        damage(sdt(b"SPCR", r.below(6) as u8, &body), r)
    };
    sweep(make, headed(spcr), |s, t| s.namespace.as_ref().map_or(0, |n| n.len()) < t.len());
}

#[test]
fn a_loaded_table_is_one_whole_table() {
    let make = |r: &mut Rng| damage(sdt(b"SSDT", 2, &r.some_bytes(64)), r);
    sweep(make, checked, |sig, t| {
        let sum = t.iter().fold(0u8, |s, b| s.wrapping_add(*b));
        sum == 0 && t[4..8] == (t.len() as u32).to_le_bytes() && sig[..] == t[..4]
    });
}
