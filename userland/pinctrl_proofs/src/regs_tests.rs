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

use nonos_pinctrl::{
    amd_alive, amd_pin_reg, intel_pad_stride, intel_padcfg0, PadError, AMD_PIN_STS, INTEL_RXSTATE,
};

#[test]
fn intel_stride_follows_the_revision() {
    // REVID bits 31:16; Linux turns on the debounce registers from 0x092.
    assert_eq!(intel_pad_stride(0x0091_0000), 8);
    assert_eq!(intel_pad_stride(0x0092_0000), 16);
    assert_eq!(intel_pad_stride(0x0110_0000), 16);
}

#[test]
fn intel_padcfg0_offsets_and_refusals() {
    assert_eq!(intel_padcfg0(0x0094_0000, 0x700, 62, 0x1_0000), Ok(0x700 + 62 * 16));
    assert_eq!(intel_padcfg0(0x0090_0000, 0x400, 55, 0x1_0000), Ok(0x400 + 55 * 8));
    assert_eq!(intel_padcfg0(u32::MAX, 0x700, 1, 0x1_0000), Err(PadError::Absent));
    assert_eq!(intel_padcfg0(0x0094_0000, 0, 1, 0x1_0000), Err(PadError::Absent));
    assert_eq!(intel_padcfg0(0x0094_0000, 0xFFF0, 1, 0x1_0000), Err(PadError::OutsideWindow));
    assert_eq!(INTEL_RXSTATE, 0b10);
}

#[test]
fn amd_pin_registers() {
    // AMDI0030's bank is 0x400 bytes: pins 0..=255.
    assert_eq!(amd_pin_reg(0, 0x400), Ok(0));
    assert_eq!(amd_pin_reg(9, 0x400), Ok(0x24));
    assert_eq!(amd_pin_reg(255, 0x400), Ok(0x3FC));
    assert_eq!(amd_pin_reg(256, 0x400), Err(PadError::OutsideWindow));
    assert_eq!(AMD_PIN_STS, 0x1_0000);
    assert_eq!(amd_alive(u32::MAX), Err(PadError::Absent));
    assert_eq!(amd_alive(0x0015_0000), Ok(()));
}
