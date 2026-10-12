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

//! The server's serial console as firmware names it, read once at boot
//! and served read-only: SPCR's port (a BMC's serial-over-LAN UART, most
//! often) and DBG2's debug ports. Whether the kernel console follows SPCR
//! is `serial_console_choice`'s, behind a switch.

use super::super::state::TableRegistry;
use super::raw_table::bytes;
use super::shared_tables::dbg2::{dbg2, Dbg2Device};
use super::shared_tables::spcr::{spcr, Spcr};
use alloc::vec::Vec;
use spin::RwLock;

static SPCR: RwLock<Option<Spcr>> = RwLock::new(None);
static DBG2: RwLock<Vec<Dbg2Device>> = RwLock::new(Vec::new());

pub fn parse_serial_console(registry: &mut TableRegistry) {
    let s = bytes(registry, b"SPCR").and_then(|t| spcr(t).ok());
    if let Some(s) = &s {
        let (i, b) = (s.interface, s.base);
        crate::log::info!(
            "[ACPI] SPCR console: {:?}, space {} {:#X}, baud {:?}",
            i,
            b.space,
            b.address,
            s.baud
        );
    }
    *SPCR.write() = s;
    *DBG2.write() = bytes(registry, b"DBG2").and_then(|t| dbg2(t).ok()).unwrap_or_default();
}

pub fn serial_console() -> Option<Spcr> {
    SPCR.read().clone()
}

pub fn debug_ports() -> Vec<Dbg2Device> {
    DBG2.read().clone()
}
