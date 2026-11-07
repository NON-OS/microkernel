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

use super::late::{due, Seen};
use super::once::Tried;
use super::order::Stage::*;
use super::order::{after_acpi, order, Stage, GRAPH, ORDER, STAGES};

const ORCHESTRATOR: &str =
    include_str!("../../../../src/userspace/init/spawn_plan/orchestrator.rs");

fn at(s: Stage) -> usize {
    ORDER.iter().position(|x| *x == s).expect("placed")
}

#[test]
fn every_stage_comes_after_what_it_needs() {
    for (stage, needs) in GRAPH {
        for need in needs {
            assert!(at(*need) < at(stage), "{need:?} before {stage:?}");
        }
    }
}

#[test]
fn every_stage_is_placed_once() {
    let mut seen = ORDER.to_vec();
    seen.sort();
    seen.dedup();
    assert_eq!(seen.len(), STAGES);
}

/// The same order every time it is computed, and the one init used before
/// the needs were declared: declaring them moved nothing.
#[test]
fn the_order_is_stable_and_is_the_one_init_used() {
    assert_eq!(order(), Some(ORDER));
    let before = [
        Virtio,
        PlatformAcpi,
        VmBus,
        VmDisplay,
        Disks,
        UsbHosts,
        Buses,
        Input,
        VmInput,
        Nics,
        VmNics,
        UsbClass,
        UsbNet,
        Storage,
        VmStorage,
        VmAudio,
        Audio,
    ];
    assert_eq!(ORDER, before);
}

/// The drivers that find what ACPI publishes come after the ACPI stage,
/// whose end is the capsule's first publish pass.
#[test]
fn what_acpi_publishes_is_up_before_its_drivers() {
    for s in [Disks, UsbHosts, Buses, Input, Nics, Audio] {
        assert!(at(PlatformAcpi) < at(s), "{s:?}");
    }
    assert!(at(UsbHosts) < at(Buses), "a CNVi card's group goes to USB first");
    assert!(at(Buses) < at(Input), "the touchpad's host and its interrupt first");
}

/// Init walks ORDER, and every stage has its spawn.
#[test]
fn init_walks_the_order_and_each_stage_spawns() {
    let flat: String = ORCHESTRATOR.split_whitespace().collect();
    assert!(flat.contains("forstageinsuper::order::ORDER{spawn_stage(stage);}"));
    for (stage, _) in GRAPH {
        assert!(flat.contains(&format!("{stage:?}=>super::")), "{stage:?}");
    }
}

const BOOT: &str = include_str!("../../../../src/userspace/init/spawn_plan/boot.rs");
const LOOP: &str = include_str!("../../../../src/userspace/init/supervisor/loop_impl.rs");

/// The stages walked again on a late publish: every one that needs the ACPI
/// stage, directly or through another; none of the virtual machines' own.
#[test]
fn the_stages_after_acpi_are_the_ones_that_need_it() {
    let after: Vec<Stage> = ORDER.into_iter().filter(|s| after_acpi(*s)).collect();
    let want = [Disks, UsbHosts, Buses, Input, Nics, UsbClass, UsbNet, Storage, Audio];
    assert_eq!(after, want);
}

#[test]
fn a_walk_is_due_when_the_pass_comes_or_the_table_grows() {
    let at = |pass_done, devices| Seen { pass_done, devices };
    assert!(due(at(false, 30), at(true, 30)), "the first pass came");
    assert!(due(at(true, 30), at(true, 31)), "a host published");
    assert!(!due(at(true, 30), at(true, 30)), "nothing new");
    assert!(!due(at(true, 31), at(true, 30)), "a device left: the supervisor's");
}

#[test]
fn each_capsule_is_tried_once_a_boot() {
    let mut t = Tried::new();
    assert!(t.first("driver_i2c_pci") && t.first("driver_i2c_hid"));
    assert!(!t.first("driver_i2c_pci"));
}

/// The HP's touchpad behind an I2C host the ACPI capsule publishes at 7 s,
/// after an EC that started late kept the first pass past init's 5 s wait:
/// init's walk at 5 s finds no host, its tick at 7 s finds the pass done
/// and the host in the table and spawns the I2C drivers, once.
#[test]
fn a_host_published_at_7_s_gets_its_drivers_at_the_next_tick() {
    let host_at = 7_000;
    let table = |t: u64| 30 + usize::from(t >= host_at);
    let mut tried = Tried::new();
    let mut spawned = Vec::new();
    let mut walk = |t: u64, tried: &mut Tried| {
        if t >= host_at {
            for name in ["driver_i2c_pci", "driver_i2c_hid"] {
                if tried.first(name) {
                    spawned.push((t, name));
                }
            }
        }
    };
    // Init's walk once its wait gave up at 5 s.
    walk(5_000, &mut tried);
    let mut last = Seen { pass_done: false, devices: table(5_000) };
    for t in (6_000..=30_000).step_by(1_000) {
        let now = Seen { pass_done: t >= host_at, devices: table(t) };
        if due(last, now) {
            last = now;
            walk(t, &mut tried);
        }
    }
    assert_eq!(spawned, [(7_000, "driver_i2c_pci"), (7_000, "driver_i2c_hid")]);
}

#[test]
fn init_walks_late_on_its_tick_and_spawns_each_name_once() {
    let flat: String = ORCHESTRATOR.split_whitespace().collect();
    assert!(flat.contains("Some(before)ifsuper::late::due(before,now)=>*last=Some(now),"));
    assert!(flat.contains(".filter(|s|super::order::after_acpi(*s))"));
    assert!(LOOP.find("spawn_plan::spawn_late();") < LOOP.find("boot_census::tick("));
    let boot: String = BOOT.split_whitespace().collect();
    assert!(boot.contains("if!TRIED.lock().first(name){return;}"));
}
