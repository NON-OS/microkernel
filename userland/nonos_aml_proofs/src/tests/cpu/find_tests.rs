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

//! Processors found by what they are, named by the id the MADT uses, and
//! their tables read through the interpreter.

use super::rig::machine;
use crate::consts::at;
use crate::cpu::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn processor_objects_and_acpi0007_devices_in_order() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let found = processors(&mut aml, &mut evaluator(&mut probe));
    let want = [("\\_PR.CPU0", 1), ("\\_PR.CPU1", 2), ("\\_SB.CPU2", 7)]
        .map(|(p, uid)| Processor { id: at(&aml, p).unwrap(), uid });
    assert_eq!(found, want, "CPU3's _STA says absent; OTHR is no processor");
}

#[test]
fn performance_and_idle_tables() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let cpu0 = at(&aml, "\\_PR.CPU0").unwrap();
    let t = cpu_tables(&mut aml, &mut evaluator(&mut probe), cpu0);
    let mhz: Vec<u32> = t.pstates.iter().map(|p| p.mhz).collect();
    assert_eq!(mhz, [2400, 1600, 800]);
    assert_eq!((t.pstates[1].milliwatts, t.pstates[1].control), (9000, 0x1000));
    let pct = t.pct.unwrap();
    assert_eq!((pct.control.space, pct.control.address), (FIXED_HARDWARE, 0x199));
    assert_eq!((pct.status.width, pct.status.address), (64, 0x198));
    assert_eq!(t.ppc, Some(1));
    let c: Vec<(u8, u32, u8)> =
        t.cstates.iter().map(|c| (c.kind, c.latency_us, c.register.space)).collect();
    assert_eq!(c, [(1, 1, FIXED_HARDWARE), (2, 50, SYSTEM_IO), (3, 150, SYSTEM_IO)]);
    assert_eq!(t.cstates[1].register.address, 0x414);
    assert_eq!(t.cpc, None);

    let cpu1 = at(&aml, "\\_PR.CPU1").unwrap();
    assert_eq!(cpu_tables(&mut aml, &mut evaluator(&mut probe), cpu1), CpuTables::default());
}

#[test]
fn cppc_levels_and_registers() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let cpu2 = at(&aml, "\\_SB.CPU2").unwrap();
    let cpc = cpu_tables(&mut aml, &mut evaluator(&mut probe), cpu2).cpc.unwrap();
    assert_eq!(cpc.revision, 3);
    let levels = [HIGHEST, NOMINAL, LOWEST_NONLINEAR, LOWEST].map(|i| cpc.integer(i));
    assert_eq!(levels, [Some(255), Some(200), Some(100), Some(10)]);
    assert_eq!(cpc.fields[GUARANTEED], CpcField::Absent, "the null register");
    let CpcField::Register(desired) = cpc.fields[DESIRED] else { panic!("desired") };
    assert_eq!((desired.space, desired.width, desired.address), (PCC, 32, 0x14));
    let tail = [REFERENCE_PERFORMANCE, LOWEST_MHZ, NOMINAL_MHZ].map(|i| cpc.integer(i));
    assert_eq!(tail, [Some(150), Some(400), Some(2000)]);
}
