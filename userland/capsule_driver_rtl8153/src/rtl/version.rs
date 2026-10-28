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

//! The chip's version read before the device is configured, as Linux
//! v6.12 reads it to choose the configuration: the PLA dword at 0xe610, up
//! to three times, since a read is safe to repeat. A chip that is not an
//! RTL8153 or RTL8153B is refused by name (model.rs) and never configured.

use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::Bus;

use super::bounds::VERSION_TRIES;
use super::model::{code_of, identify, Model};
use super::ocp::read;
use super::reg::pla32;
use super::regs::pla::TX_CONFIG;

/// ENODEV: a chip this driver does not drive.
pub const E_NODEV: i32 = -19;

pub fn read_version<B: Bus>(bus: &mut B) -> Result<Model, (&'static str, i32)> {
    let mut last = E_IO;
    for _ in 0..VERSION_TRIES {
        match read(bus, pla32(TX_CONFIG)) {
            Ok(dword) => return identify(code_of(dword)).map_err(|why| (why, E_NODEV)),
            Err(e) => last = e,
        }
    }
    Err(("version register unread", last))
}
