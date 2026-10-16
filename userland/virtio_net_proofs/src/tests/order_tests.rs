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

//! Orders that live in bring-up and teardown code the host cannot run (it
//! calls the broker), checked in the shipping source text.

const SRC: &str = "../../../capsule_driver_virtio_net/src/";
const MODERN: &str = include_str!("../../../capsule_driver_virtio_net/src/setup/modern/run.rs");
const LEGACY: &str = include_str!("../../../capsule_driver_virtio_net/src/setup/legacy.rs");
const PCI: &str = include_str!("../../../capsule_driver_virtio_net/src/setup/pci.rs");
const DRIVER: &str = include_str!("../../../capsule_driver_virtio_net/src/setup/driver.rs");
const QUEUES: &str = include_str!("../../../capsule_driver_virtio_net/src/init/program_queues.rs");
const PROGRAM: &str =
    include_str!("../../../capsule_driver_virtio_net/src/setup/modern/program.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("`{needle}` not found"))
}

/// The line that defines `name` in `src`.
fn line<'a>(src: &'a str, name: &str) -> &'a str {
    src.lines().find(|l| l.contains(&format!("const {name}:"))).expect(name)
}

// A device a previous instance left live holds its rings until it is reset
// and given these. Bring-up hands Bus Master to the queue step, which sets
// it after every refusal (`master_legacy_tests`, `master_modern_tests`).
#[test]
fn a_device_is_reset_and_on_this_drivers_rings_before_it_may_master_the_bus() {
    assert!(!line(PCI, "DECODE").contains("BUS_MASTER"), "decoding sets Bus Master");
    assert!(line(PCI, "MASTER").contains("BUS_MASTER"));
    assert!(at(MODERN, "pci::DECODE") < at(MODERN, "start_until(&common"));
    let master = "let master = || pci::command(dev.device_id, claim_epoch, pci::MASTER);";
    for (src, reset) in [(MODERN, "start_until(&common"), (LEGACY, "negotiate(regs)")] {
        assert_eq!(src.matches("pci::MASTER").count(), 1, "Bus Master outside the queue step");
        assert!(at(src, reset) < at(src, master), "Bus Master before the reset");
        assert!(!src.contains("master()"), "Bus Master before the queues");
        assert!(at(src, "&dma, master)?") < at(src, "driver_ok("), "DRIVER_OK before Bus Master");
    }
    assert!(at(QUEUES, "program_queue(regs, Q_TX") < at(QUEUES, "master()?"), "rings after BME");
    assert!(at(PROGRAM, "both(common, notify, rx_ring, tx_ring)") < at(PROGRAM, "master()"));
}

// Teardown stops the queues, then hands the device back: the broker clears
// bus mastering and only then frees the grants. Nothing is unmapped first.
#[test]
fn teardown_resets_the_device_and_never_unmaps_dma_itself() {
    assert!(at(DRIVER, "self.transport.reset()") < at(DRIVER, "mk_device_release("));
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests").join(SRC);
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("driver source") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let text = std::fs::read_to_string(&path).expect("read");
                assert!(!text.contains("mk_dma_unmap"), "{} unmaps DMA", path.display());
            }
        }
    }
}
