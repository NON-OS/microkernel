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
//! The PCI configuration writes Linux makes for each controller, and the
//! masked update that keeps them within the bits the broker allows.

#[test]
fn gemini_lake_clears_tcsel_and_nosnoop_and_ungates_the_clock_around_reset() {
    use crate::controller::intel::pci_quirks;
    let q = pci_quirks(0x8086, 0x3198);
    assert!(q.clear_tcsel && q.clear_nosnoop && q.gate_cgctl && !q.amd_snoop);
    let skl = pci_quirks(0x8086, 0x9d70);
    assert!(skl.gate_cgctl && skl.clear_nosnoop, "Skylake-LP is in the family");
}

#[test]
fn qemu_ich9_only_has_tcsel_cleared() {
    use crate::controller::intel::pci_quirks;
    let q = pci_quirks(0x8086, 0x293e);
    assert!(q.clear_tcsel);
    assert!(!q.clear_nosnoop && !q.gate_cgctl && !q.amd_snoop, "ICH has no DEVC or CGCTL");
}

#[test]
fn amd_turns_snooping_on_and_writes_nothing_intel() {
    use crate::controller::intel::pci_quirks;
    let q = pci_quirks(0x1022, 0x15e3);
    assert!(q.amd_snoop);
    assert!(!q.clear_tcsel && !q.clear_nosnoop && !q.gate_cgctl, "AZX_DCAPS_NO_TCSEL on AMD");
    let gpu = pci_quirks(0x1002, 0xaa20);
    assert_eq!(gpu, Default::default(), "a graphics card's HDMI function is left as it is");
}

#[test]
fn a_masked_update_changes_only_the_bits_the_broker_allows() {
    use crate::controller::intel::{with_bits, AMD_ENABLE_SNOOP, AMD_SNOOP_MASK};
    assert_eq!(with_bits(0x1207, 0x0007, 0), 0x1200, "TCSEL cleared, the byte above kept");
    assert_eq!(with_bits(0x0840, 0x0040, 0), 0x0800, "MISCBDCGE off, nothing else");
    assert_eq!(with_bits(0x0800, 0x0040, 0x0040), 0x0840, "and back on");
    assert_eq!(with_bits(0xab05, AMD_SNOOP_MASK, AMD_ENABLE_SNOOP), 0xab02);
    for (cur, mask, want) in [(0xffffu16, 0x7u16, 0u16), (0x0000, 0x0800, 0), (0x1234, 0x40, 0x40)]
    {
        assert_eq!((with_bits(cur, mask, want) ^ cur) & !mask, 0, "a bit outside the mask moved");
    }
}
