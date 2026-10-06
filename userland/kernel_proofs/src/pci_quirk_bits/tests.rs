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

use super::quirk_bits::{only, writable, Ident};

const HDA: Ident = Ident { vendor: 0x8086, class: 0x04, subclass: 0x03, pcie_cap: None };
const DSP: Ident = Ident { vendor: 0x8086, class: 0x04, subclass: 0x01, pcie_cap: None };
const AMD_HDA: Ident = Ident { vendor: 0x1022, class: 0x04, subclass: 0x03, pcie_cap: None };
const WIFI: Ident = Ident { vendor: 0x10EC, class: 0x02, subclass: 0x80, pcie_cap: Some(0x70) };
const NVME: Ident = Ident { vendor: 0x8086, class: 0x01, subclass: 0x08, pcie_cap: Some(0x70) };

#[test]
fn intel_audio_gets_exactly_the_bits_snd_hda_intel_writes() {
    for ident in [HDA, DSP] {
        assert_eq!(writable(&ident, 0x44), Some(0x0007), "TCSEL");
        assert_eq!(writable(&ident, 0x48), Some(0x0040), "CGCTL MISCBDCGE");
        assert_eq!(writable(&ident, 0x78), Some(0x0800), "DEVC NOSNOOP");
        assert_eq!(writable(&ident, 0x42), None, "AMD's snoop register is not Intel's");
    }
}

#[test]
fn amd_audio_gets_its_snoop_bits_only() {
    assert_eq!(writable(&AMD_HDA, 0x42), Some(0x0007));
    for offset in [0x44, 0x48, 0x78] {
        assert_eq!(writable(&AMD_HDA, offset), None);
    }
}

#[test]
fn a_network_function_gets_completion_timeout_and_nothing_else() {
    assert_eq!(writable(&WIFI, 0x70 + 0x28), Some(0x001F));
    for offset in [0x70, 0x70 + 0x08, 0x70 + 0x10, 0x70 + 0x2A, 0x44, 0x48, 0x78] {
        assert_eq!(writable(&WIFI, offset), None, "offset {offset:#x}");
    }
    let no_cap = Ident { pcie_cap: None, ..WIFI };
    assert_eq!(writable(&no_cap, 0x98), None, "no PCIe capability, no DEVCTL2");
}

#[test]
fn every_other_class_and_register_stays_refused() {
    for offset in [0x00, 0x04, 0x10, 0x3C, 0x42, 0x44, 0x48, 0x78, 0x98] {
        assert_eq!(writable(&NVME, offset), None, "storage {offset:#x}");
    }
    let gpu = Ident { vendor: 0x8086, class: 0x03, subclass: 0x00, pcie_cap: None };
    assert_eq!(writable(&gpu, 0x48), None);
}

#[test]
fn a_write_may_move_only_the_allowed_bits() {
    assert!(only(0x0040, 0x1234 & !0x0040, 0x1234), "clear MISCBDCGE");
    assert!(only(0x0007, 0xAB00, 0xAB07), "TCSEL to 0");
    assert!(!only(0x0040, 0x1235, 0x1234), "a neighbouring bit");
    assert!(!only(0x001F, 0x0020, 0x0000), "past the completion timeout field");
    assert!(only(0x001F, 0x0010, 0x0000), "completion timeout disable");
}
