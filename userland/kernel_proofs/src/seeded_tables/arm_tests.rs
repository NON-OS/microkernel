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

//! The Arm MADT's GIC structures and PCCT's subspaces: structures of every
//! kind and of none, lengths that lie, tables cut short; and a flood past
//! each cap, which an undamaged table must stop at exactly.

use super::gen::{damage, honest, sdt};
use super::sweep::{body, headed, sweep};
use crate::arm_acpi::madt::{madt_gic, MAX_STRUCTURES};
use crate::arm_acpi::pcct::{pcct, MAX_SUBSPACES};

const GIC: [(u8, usize); 5] = [(0x0B, 80), (0x0C, 22), (0x0E, 14), (0x0F, 18), (0x7A, 6)];
const PCC: [(u8, usize); 6] = [(0, 60), (1, 60), (2, 88), (3, 162), (5, 94), (0x40, 8)];

#[test]
fn madt_gic_structures_load_or_are_refused() {
    sweep(
        |r| damage(sdt(b"APIC", 5, &body(r, 8, &GIC)), r),
        headed(madt_gic),
        |g, t| {
            let kept = g.gicc.len() + g.gicd.len() + g.gicr.len() + g.its.len();
            kept <= MAX_STRUCTURES && 44 + 16 * kept <= t.len()
        },
    );
}

#[test]
fn madt_gic_stops_at_its_cap() {
    let gicd = honest(0x0C, &[0; 22]);
    let t = sdt(b"APIC", 5, &[vec![0; 8], gicd.repeat(MAX_STRUCTURES + 9)].concat());
    assert_eq!(madt_gic(&t).unwrap().gicd.len(), MAX_STRUCTURES);
}

#[test]
fn pcct_subspaces_load_or_are_refused() {
    sweep(
        |r| damage(sdt(b"PCCT", 2, &body(r, 12, &PCC)), r),
        headed(pcct),
        |p, t| p.subspaces.len() <= MAX_SUBSPACES && 48 + 2 * p.subspaces.len() <= t.len(),
    );
}

#[test]
fn pcct_stops_at_its_cap() {
    let t = sdt(b"PCCT", 2, &[vec![0; 12], honest(0x40, &[]).repeat(MAX_SUBSPACES + 40)].concat());
    assert_eq!(pcct(&t).unwrap().subspaces.len(), MAX_SUBSPACES);
}
