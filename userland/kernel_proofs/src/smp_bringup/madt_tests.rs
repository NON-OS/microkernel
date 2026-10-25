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

use super::madt_plan::MadtCpu;
use super::madt_support::{lapic, run, x2};

/* ---- which MADT entries become APs ---- */

#[test]
fn gemini_lake_four_cores_dense_ids() {
    let (aps, c) = run(&[lapic(0), lapic(2), lapic(4), lapic(6)], 0, false, 256);
    assert_eq!(aps, [2, 4, 6]);
    assert_eq!(c.accepted, 3);
}

#[test]
fn hybrid_alder_lake_sparse_ids_and_disabled_slots() {
    // 6P (HT) + 8E: P ids 0..11 in pairs, E ids 0x40..0x47, plus hotplug
    // slots firmware lists disabled.
    let mut e: Vec<MadtCpu> = (0..12).map(lapic).collect();
    e.extend((0x40..0x48).map(lapic));
    e.extend((0x60..0x64).map(|id| MadtCpu { apic_id: id, enabled: false, x2apic_entry: false }));
    let (aps, c) = run(&e, 0, false, 256);
    assert_eq!(aps.len(), 19);
    assert!(aps.contains(&0x47));
    assert_eq!(c.disabled, 4);
}

#[test]
fn bsp_not_first_and_not_zero() {
    let (aps, _) = run(&[lapic(0), lapic(1), lapic(8), lapic(9)], 8, false, 256);
    assert_eq!(aps, [0, 1, 9]);
}

#[test]
fn duplicate_lapic_and_x2apic_entries_start_once() {
    let e = [lapic(0), lapic(2), x2(0), x2(2), x2(4)];
    let (aps, c) = run(&e, 0, true, 256);
    assert_eq!(aps, [2, 4]);
    assert_eq!(c.duplicate, 1);
}
