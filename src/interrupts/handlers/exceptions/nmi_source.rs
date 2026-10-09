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

use x86_64::instructions::port::Port;

/// System control port A - used for system reset and A20 gate control
pub const SYSTEM_CONTROL_PORT_A: u16 = 0x92;
/// System control port B - used for NMI source identification
pub const SYSTEM_CONTROL_PORT_B: u16 = 0x61;

#[derive(Debug, Clone, Copy)]
pub enum NmiSource {
    MemoryParity,
    IoChannelCheck,
    Watchdog,
    Unknown,
}

pub(super) fn identify_nmi_source() -> NmiSource {
    /*
     * SAFETY: Reading system control port B to determine NMI source
     */
    let status = unsafe {
        let mut port = Port::<u8>::new(SYSTEM_CONTROL_PORT_B);
        port.read()
    };

    if (status & 0x80) != 0 {
        NmiSource::MemoryParity
    } else if (status & 0x40) != 0 {
        NmiSource::IoChannelCheck
    } else {
        NmiSource::Unknown
    }
}
