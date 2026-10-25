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

/* ---- which MADT entries become APs: what is refused ---- */

#[test]
fn placeholders_are_not_processors() {
    let (aps, c) = run(&[lapic(0), lapic(0xFF), x2(u32::MAX), lapic(1)], 0, true, 256);
    assert_eq!(aps, [1]);
    assert_eq!(c.invalid, 2);
}

#[test]
fn ids_past_0xfe_need_x2apic() {
    let e = [x2(0), x2(0x100), x2(0x101), lapic(3)];
    let (aps, c) = run(&e, 0, false, 256);
    assert_eq!(aps, [3]);
    assert_eq!(c.unaddressable, 2);
    let (aps, _) = run(&e, 0, true, 256);
    assert_eq!(aps, [0x100, 0x101, 3]);
}

#[test]
fn more_cpus_than_max_are_counted_not_overrun() {
    let e: Vec<MadtCpu> = (0..40).map(lapic).collect();
    let (aps, c) = run(&e, 0, false, 16);
    assert_eq!(aps.len(), 15, "15 APs plus the boot CPU is 16");
    assert_eq!(c.over_limit, 24);
}
