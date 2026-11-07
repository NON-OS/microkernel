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

//! The command register the boot pass leaves on each function (K10a),
//! pure: its decoders on, so it can be enumerated and claimed, and Bus
//! Master never set. A function firmware left mastering is stopped, so an
//! engine it left armed (a PXE NIC still receiving) masters nothing until
//! a driver has stopped it and turned Bus Master on through its claim. A
//! display keeps the bit as found: the firmware's scanout, which the boot
//! console draws on, reads through it. kernel_proofs::bus::boot_command.

pub(super) const IO_SPACE: u16 = 1 << 0;
pub(super) const MEMORY_SPACE: u16 = 1 << 1;
pub(super) const BUS_MASTER: u16 = 1 << 2;
pub(super) const CLASS_DISPLAY: u8 = 0x03;

pub(super) const fn boot_command(found: u16, class: u8) -> u16 {
    let decoding = found | IO_SPACE | MEMORY_SPACE;
    if class == CLASS_DISPLAY {
        decoding
    } else {
        decoding & !BUS_MASTER
    }
}
