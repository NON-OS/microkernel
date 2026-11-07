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

//! ACPI mode entered through FADT's SMI command port, with a bounded wait
//! for the firmware's answer.

use crate::events_io::PortIo;
use crate::sys::log::Line;
use nonos_aml::events::{enter_acpi_mode, Fadt};

/// How long firmware gets to answer ACPI_ENABLE.
const MODE_WAIT_MS: u64 = 3000;

pub(super) fn enter_mode(f: &Fadt) {
    let r = enter_acpi_mode(f, &mut PortIo, MODE_WAIT_MS);
    if let Some(l) = crate::trace::line() {
        let said: &[u8] = match &r {
            Ok(true) => b"entered",
            Ok(false) => b"already on",
            Err(_) => b"not entered",
        };
        l.str(b"acpi mode ").str(said).end();
    }
    match r {
        Ok(true) => Line::new(b"entered ACPI mode").end(),
        Ok(false) => Line::new(b"already in ACPI mode").end(),
        Err(e) => {
            Line::new(b"ACPI mode not entered: ").str(alloc::format!("{e:?}").as_bytes()).end()
        }
    }
}
