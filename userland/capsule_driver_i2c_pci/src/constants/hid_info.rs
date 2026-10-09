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

// The kernel's I2C-HID record (src/hardware/broker/acpi_i2c/hid_record.rs)
// carries interrupt and descriptor facts in bar 0's flags byte; the bits
// this driver acts on.
pub const HID_INFO_GPIO: u8 = 1 << 0;
pub const HID_INFO_ACTIVE_HIGH: u8 = 1 << 3;
pub const HID_INFO_TEN_BIT: u8 = 1 << 5;
// The kernel's GPIO controller record (src/hardware/broker/acpi_gpio/
// record.rs) carries the controller's eight `_HID` bytes in bar 5's base.
pub const GPIO_HID_BAR: usize = 5;

// Intel pinctrl community registers (Linux pinctrl-intel.c): PADBAR at 0x00C
// holds the offset of the pad configuration array; PADCFG0 bit 1 is the
// pad's input level after the glitch filter, before RX inversion.
pub const GPIO_PADBAR: u64 = 0x00C;
pub const GPIO_RXSTATE: u32 = 1 << 1;
// Broxton/Gemini Lake pads have two configuration dwords (no debounce
// register), so consecutive pads sit eight bytes apart.
pub const BXT_PAD_STRIDE: u64 = 8;

/// Where the pad configuration register of `pin` sits in a Broxton-family
/// community window of `window` bytes whose PADBAR reads `padbar`, or None
/// when it falls outside the window (a wrong community, or a PADBAR read
/// from dead MMIO).
pub fn bxt_pad_offset(padbar: u32, pin: u16, window: u64) -> Option<u64> {
    let off = u64::from(padbar).checked_add(u64::from(pin) * BXT_PAD_STRIDE)?;
    (padbar != 0 && padbar != u32::MAX && off + 4 <= window).then_some(off)
}
