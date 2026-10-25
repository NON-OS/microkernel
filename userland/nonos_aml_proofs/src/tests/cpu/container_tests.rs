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

//! The container walk: a processor's own level first, then each
//! ACPI0010 around it, stopping where a container cannot be named.

use super::lpi_rig::machine;
use crate::consts::at;
use crate::cpu::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

fn levels(path: &str) -> Vec<Option<u64>> {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let id = at(&aml, path).unwrap();
    let l = idle_levels(&mut aml, &mut evaluator(&mut probe), id);
    l.iter().map(|l| l.container.map(|(_, uid)| uid)).collect()
}

#[test]
fn the_hierarchy_nearest_first() {
    assert_eq!(levels("\\_SB.SYS0.CLU0.C000"), [None, Some(0x10), Some(0x100)]);
    assert_eq!(levels("\\_SB.SYS0.CLU1.C100"), [None], "CLU1 has no _UID");
    assert_eq!(levels("\\_SB.SYS0.GRP2.C200"), [None, Some(0x100)], "GRP2 is passed over");
    assert_eq!(levels("\\_SB.C300"), [None]);
    assert!(levels("\\_SB.SYS0").len() == 1, "a container's own level, then none above");
}

#[test]
fn tables_carry_the_levels() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let id = at(&aml, "\\_SB.SYS0.CLU0.C001").unwrap();
    let t = cpu_tables(&mut aml, &mut evaluator(&mut probe), id);
    let entries: Vec<LpiEntry> = t.lpi[1].lpi.states.iter().map(|s| s.entry).collect();
    assert_eq!(entries, [LpiEntry::Integer(0x10000), LpiEntry::Integer(0x20000)]);
    assert_eq!(t.lpi[0].lpi.states[1].parent_state, 2);
    assert!(t.cstates.is_empty());
}
