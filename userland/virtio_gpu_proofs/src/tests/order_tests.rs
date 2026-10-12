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

use super::recover_order_tests::sources;

const SEQUENCE: &str = include_str!("../../../capsule_driver_virtio_gpu/src/setup/sequence.rs");
const TRANSPORT: &str = include_str!("../../../capsule_driver_virtio_gpu/src/setup/transport.rs");
const PCI: &str = include_str!("../../../capsule_driver_virtio_gpu/src/setup/pci.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("`{needle}` not found"))
}

// A bring-up that fails once the window is mapped resets the device, and
// only then is the claim released (plan/50 C2). No grant is unmapped by the
// driver anywhere: the release takes them once bus mastering is off.
#[test]
fn a_failed_bring_up_resets_the_device_and_never_unmaps_dma_itself() {
    assert!(at(SEQUENCE, "live::run(") < at(SEQUENCE, "reset_until(&window.common"));
    assert!(at(SEQUENCE, "claimed(dev, claim_epoch)") < at(SEQUENCE, "mk_device_release("));
    for (path, text) in sources("") {
        assert!(!text.contains("mk_dma_unmap"), "{} unmaps DMA", path.display());
    }
}

// A device left live by a previous instance holds its ring addresses until
// it is reset; it may not master the bus before that, nor see a ring.
#[test]
fn a_device_is_reset_before_it_may_master_the_bus() {
    let decode = PCI.lines().find(|l| l.contains("const DECODE:")).expect("DECODE");
    assert!(!decode.contains("BUS_MASTER"), "decoding sets Bus Master");
    assert!(at(SEQUENCE, "pci::DECODE") < at(SEQUENCE, "mmio::map("));
    assert!(at(TRANSPORT, "negotiate(&common)") < at(TRANSPORT, "pci::MASTER"));
    assert!(at(TRANSPORT, "pci::MASTER") < at(TRANSPORT, "program(&common"));
}
