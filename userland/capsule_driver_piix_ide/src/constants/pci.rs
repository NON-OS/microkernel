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

//! The controllers and their windows. In compatibility mode (programming
//! interface bit 0 or 2 clear) a channel's command and control blocks sit at
//! the ISA addresses and its BARs read zero; the kernel's device table lists
//! them in BAR 0..3 all the same (`src/hardware/broker/legacy/attach.rs`),
//! so the driver reads every window from the record either way.

pub const INTEL_VENDOR_ID: u16 = 0x8086;
/// 82371SB PIIX3 IDE: QEMU's `pc` machine and VirtualBox's PIIX3 chipset.
pub const PIIX3_IDE_DEVICE_ID: u16 = 0x7010;
/// 82371AB/EB/MB PIIX4 IDE: VirtualBox's default, Hyper-V Generation 1.
pub const PIIX4_IDE_DEVICE_ID: u16 = 0x7111;
pub const CLASS_STORAGE: u8 = 0x01;
pub const SUBCLASS_IDE: u8 = 0x01;

/// Channel `c`'s command block is BAR `2c`, its control block BAR `2c + 1`.
pub const CHANNELS: usize = 2;
pub const fn command_bar(channel: usize) -> u8 {
    (channel * 2) as u8
}
pub const fn control_bar(channel: usize) -> u8 {
    (channel * 2 + 1) as u8
}
/// The bus master window, both channels' engines in sixteen ports.
pub const BUS_MASTER_BAR: u8 = 4;
pub const COMMAND_MIN_PORTS: u16 = 8;
pub const CONTROL_MIN_PORTS: u16 = 1;
pub const BUS_MASTER_MIN_PORTS: u16 = 16;

/// A native-mode control BAR is four ports with the register at +2; the
/// kernel lists a compatibility-mode one as the register alone (0x3F6,
/// 0x376), so the floppy controller's ports beside it are never granted.
pub const fn control_offset(window: u16) -> u16 {
    if window >= 4 {
        2
    } else {
        0
    }
}
