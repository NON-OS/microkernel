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


//! The kernel asks the rule before it starts either I2C capsule: both spawns
//! come after the presence check, which takes every record the broker lists
//! through inventory::i2c_host.

const SPAWN: &str = include_str!("../../../../src/userspace/init/spawn_plan/drivers_bus/i2c.rs");
const PRESENT: &str = include_str!("../../../../src/hardware/inventory/i2c_present.rs");

#[test]
fn both_capsules_start_only_after_the_host_is_found() {
    let gate = SPAWN.find("if !crate::hardware::inventory::i2c_host_present() {").expect("no gate");
    let refused = SPAWN[gate..].find("return;\n    }").expect("the gate does not return");
    for spawn in ["    spawn_i2c_pci();\n", "    spawn_i2c_hid();\n"] {
        let at = SPAWN.find(spawn).expect("spawn missing");
        assert!(at > gate + refused, "{spawn:?} runs before the gate");
    }
}

#[test]
fn the_presence_check_reads_every_listed_record_through_the_rule() {
    assert!(PRESENT.contains("list().iter().any(|r| {"));
    let asked = "i2c_host(r.bus_kind, r.vendor, r.device, r.class, (r.pci_class, r.pci_subclass))";
    assert!(PRESENT.contains(asked));
}
