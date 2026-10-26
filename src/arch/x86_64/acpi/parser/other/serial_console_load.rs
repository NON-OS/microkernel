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

//! The server's serial console as firmware names it in SPCR (a BMC's
//! serial-over-LAN UART, most often), read once at boot and logged. The
//! kernel console stays on COM1.

use super::super::state::TableRegistry;
use super::raw_table::bytes;
use super::shared_tables::spcr::spcr;

pub fn parse_serial_console(registry: &mut TableRegistry) {
    let Some(s) = bytes(registry, b"SPCR").and_then(|t| spcr(t).ok()) else { return };
    let (i, b) = (s.interface, s.base);
    crate::log::info!(
        "[ACPI] SPCR console: {:?}, space {} {:#X}, baud {:?}",
        i,
        b.space,
        b.address,
        s.baud
    );
}
