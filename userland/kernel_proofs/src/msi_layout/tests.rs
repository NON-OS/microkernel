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

use super::layout::{disable, enable_one, layout, open_first, MsiLayout, CTRL_ENABLE};

// PCI Local Bus 3.0, Figures 6-9 to 6-12, for a capability at 0x50.
#[test]
fn the_four_capability_shapes_match_the_spec() {
    let a32 = layout(0x50, false, false);
    assert_eq!(
        a32,
        MsiLayout { control: 0x52, address_lo: 0x54, address_hi: None, data: 0x58, mask: None }
    );
    let a64 = layout(0x50, true, false);
    assert_eq!(
        a64,
        MsiLayout {
            control: 0x52,
            address_lo: 0x54,
            address_hi: Some(0x58),
            data: 0x5C,
            mask: None
        }
    );
    assert_eq!(layout(0x50, false, true).mask, Some(0x5C));
    assert_eq!(layout(0x50, true, true).data, 0x5C);
    assert_eq!(layout(0x50, true, true).mask, Some(0x60));
}

// lspci: Intel HDA "[60] MSI: Count=1/1 Maskable- 64bit+", Intel AHCI
// "[80] MSI: Count=1/1 Maskable- 64bit-", RTL8821CE "[50] MSI: 64bit+".
#[test]
fn msi_only_parts_get_their_data_register_right() {
    assert_eq!(layout(0x60, true, false).data, 0x6C);
    assert_eq!(layout(0x80, false, false).data, 0x88);
    assert_eq!(layout(0x80, false, false).address_hi, None);
    assert_eq!(layout(0x50, true, false).address_hi, Some(0x58));
}

#[test]
fn enabling_keeps_the_capability_bits_and_asks_for_one_message() {
    // Maskable, 64-bit, MMC = 3 (8 vectors), firmware left MME = 2.
    let ctrl = 0x0100 | 0x0080 | (3 << 1) | (2 << 4);
    let on = enable_one(ctrl);
    assert_eq!(on & CTRL_ENABLE, CTRL_ENABLE);
    assert_eq!(on & (0x7 << 4), 0);
    assert_eq!(on & !(CTRL_ENABLE | (0x7 << 4)), ctrl & !(0x7 << 4));
    assert_eq!(disable(on), on & !CTRL_ENABLE);
    assert_eq!(disable(on) & (0x7 << 1), 3 << 1);
}

#[test]
fn only_vector_zero_is_left_unmasked() {
    assert_eq!(open_first(0), 0);
    assert_eq!(open_first(1), 0b10);
    assert_eq!(open_first(3), 0xFE);
    assert_eq!(open_first(5), 0xFFFF_FFFE);
    // 6 and 7 are reserved encodings; no bit past the 32 is invented.
    assert_eq!(open_first(7), 0xFFFF_FFFE);
}
