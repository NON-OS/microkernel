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

//! Intel's USB Bluetooth controllers, the Bluetooth half of the AX201,
//! AX210, AX211 and BE200 cards, start in a bootloader and do nothing a
//! host asks until their firmware has been sent and booted. The facts are
//! Linux v6.12's drivers/bluetooth/btintel.c and btusb.c
//! (`btintel_setup_combined` and what it calls); the code is this tree's.
//! `setup` runs before the standard bring-up, which then resets the
//! controller again and reads what it is.

mod ask;
mod boot;
pub mod carried;
pub mod cmds;
pub mod ddc;
mod download;
mod end;
mod files;
mod flow;
mod ids;
pub mod legacy;
pub mod name;
pub mod send;
pub mod sfi;
mod steps;
pub mod tlv;
mod wire;

use nonos_bt_hci::usb::{UsbHci, UsbPipes};
use nonos_bt_hci::{HciError, Host};

pub use wire::Say;

/// Intel's USB vendor id.
pub const VID: u16 = 0x8087;

/// The firmware download for Intel's product `pid`, when it is one Linux
/// marks BTUSB_INTEL_COMBINED; any other is left to the standard bring-up.
/// Each step is said through `say`, a whole line at a time.
pub fn setup<P: UsbPipes>(host: &mut Host<UsbHci<P>>, pid: u16, say: Say) -> Result<(), HciError> {
    if !ids::combined(pid) {
        return Ok(());
    }
    flow::run(&mut wire::Wire::new(host.transport(), say))
}
