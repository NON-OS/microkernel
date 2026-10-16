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
const PROGRAM: &str = include_str!("../../../capsule_driver_virtio_rng/src/init/program.rs");
const QUEUE: &str = include_str!("../../../capsule_driver_virtio_rng/src/setup/modern/queue.rs");

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
// it is reset and given this driver's ring; it may not master the bus
// before both. Bring-up hands its one Bus Master command to the queue step,
// which runs it after the ring and every refusal, then sets DRIVER_OK
// (`master_legacy_tests` and `master_modern_tests` run that step).
#[test]
fn a_device_is_reset_and_on_this_drivers_ring_before_it_may_master_the_bus() {
    let decode = PCI.lines().find(|l| l.contains("const DECODE:")).expect("DECODE");
    assert!(!decode.contains("BUS_MASTER"), "decoding sets Bus Master");
    let master = "let master = || pci::command(dev.device_id, claim_epoch, pci::MASTER);";
    for (src, reset) in [(LEGACY, "negotiate(regs)?"), (MODERN, "start_until(&common")] {
        assert_eq!(src.matches("pci::MASTER").count(), 1, "Bus Master outside the queue step");
        assert!(at(src, reset) < at(src, master), "Bus Master before the reset");
        assert!(!src.contains("master()"), "Bus Master before the queue step");
        assert!(at(src, master) < at(src, "dma.ring.device_addr, master)?"));
    }
    let steps = [(PROGRAM, "program(regs, queue_phys)?", "driver_ok(regs);")];
    for (src, ring, ok) in steps.into_iter().chain([(QUEUE, "setup(common,", "driver_ok(common);")])
    {
        assert!(at(src, ring) < at(src, "master()?"), "Bus Master before the ring");
        assert!(at(src, "master()?") < at(src, ok), "DRIVER_OK before Bus Master");
    }
}

#[test]
fn the_modern_transport_is_taken_whenever_the_function_presents_it() {
    assert!(PROBE.contains("modern_caps(&dev.bars"));
    assert!(!PROBE.contains("wants_probe"), "a transitional function's capabilities go unread");
}
