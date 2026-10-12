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

//! Whether the kernel console moves to the port SPCR names. The console
//! stays on COM1 unless `CONSOLE_FROM_SPCR` is on; with it on,
//! `spcr_console_port` names SPCR's 16550 in I/O space for the console to
//! move to. It is off, and nothing moves the console yet.

use super::serial_console::spcr::Interface;
use super::serial_console_load::serial_console;

/// The owner's switch for taking the kernel console from SPCR.
pub const CONSOLE_FROM_SPCR: bool = false;

pub fn spcr_console_port() -> Option<u16> {
    let s = serial_console().filter(|_| CONSOLE_FROM_SPCR)?;
    let uart = matches!(
        s.interface,
        Interface::Ns16550 | Interface::Ns16550Subset | Interface::Ns16550Gas
    );
    (uart && s.base.space == 1).then(|| u16::try_from(s.base.address).ok()).flatten()
}
