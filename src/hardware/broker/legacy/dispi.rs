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

//! VirtualBox's VBoxVGA: the DISPI index and data ports, which it decodes at
//! fixed addresses outside any BAR (VB src/VBox/Devices/Graphics/DevVGA.cpp
//! L6799-6800). The same device id with an I/O BAR is VBoxSVGA, whose
//! registers sit in that BAR and which needs nothing fixed.

use super::compat::Ports;

pub(crate) const VBOX_VENDOR: u16 = 0x80EE;
pub(crate) const VBOX_VGA: u16 = 0xBEEF;

// Index at 0x1CE, data at 0x1CF. A 16-bit access to the data port also
// spans 0x1D0, so the window is three ports wide.
pub(crate) const DISPI: Ports = Ports { base: 0x1CE, count: 3 };

pub(crate) const fn dispi(vendor: u16, device: u16, has_io_bar: bool) -> Option<Ports> {
    if vendor == VBOX_VENDOR && device == VBOX_VGA && !has_io_bar {
        Some(DISPI)
    } else {
        None
    }
}
