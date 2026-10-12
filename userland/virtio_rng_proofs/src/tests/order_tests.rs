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

//! Orders that live in bring-up, which calls the kernel and so cannot run
//! on the host, checked in the shipping text.

const LEGACY: &str = include_str!("../../../capsule_driver_virtio_rng/src/setup/legacy.rs");
const MODERN: &str = include_str!("../../../capsule_driver_virtio_rng/src/setup/modern/run.rs");
const PCI: &str = include_str!("../../../capsule_driver_virtio_rng/src/setup/pci.rs");
const PROBE: &str = include_str!("../../../capsule_driver_virtio_rng/src/transport/probe.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("`{needle}` not found"))
}

// A bring-up that fails once the register window is mapped resets the
// device; `sequence::run` then releases the claim.
#[test]
fn a_failed_bring_up_resets_the_device_before_the_claim_goes() {
    assert!(at(LEGACY, "if live.is_err()") < at(LEGACY, "Transport::Legacy(regs).reset()"));
    assert!(at(MODERN, "if live.is_err()") < at(MODERN, "reset_until(&window.common"));
}

// A device left live by a previous instance holds its ring addresses until
// it is reset; it may not master the bus before that, nor see a ring.
#[test]
fn a_device_is_reset_before_it_may_master_the_bus() {
    let decode = PCI.lines().find(|l| l.contains("const DECODE:")).expect("DECODE");
    assert!(!decode.contains("BUS_MASTER"), "decoding sets Bus Master");
    assert!(at(LEGACY, "negotiate(regs)") < at(LEGACY, "pci::MASTER"));
    assert!(at(LEGACY, "pci::MASTER") < at(LEGACY, "program(regs"));
    assert!(at(MODERN, "start_until(&common") < at(MODERN, "pci::MASTER"));
    assert!(at(MODERN, "pci::MASTER") < at(MODERN, "queue::setup("));
}

#[test]
fn the_modern_transport_is_taken_whenever_the_function_presents_it() {
    assert!(PROBE.contains("modern_caps(&dev.bars"));
    assert!(!PROBE.contains("wants_probe"), "a transitional function's capabilities go unread");
}
