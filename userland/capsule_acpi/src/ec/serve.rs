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

//! The EC's event (ACPI 6.5, 12.6): while SCI_EVT is set, ask the EC which
//! event with QR_EC and run that _Qxx. Bounded, so an EC that keeps
//! answering cannot hold the capsule.

use super::state::Ec;
use crate::boot::evaluate;
use crate::events_io::PortIo;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::ec::{pending, query};
use nonos_aml::exec::{Fault, FaultKind};
use nonos_aml::load::Aml;

/// More events at once than any EC queues.
const MAX_QUERIES: u32 = 32;

pub fn serve(ec: &Ec, aml: &mut Aml, host: &mut CapsuleHost) -> Result<(), Fault> {
    let io_fault = Fault::new(FaultKind::Region, 0, 0);
    for _ in 0..MAX_QUERIES {
        if !pending(&ec.ports, &mut PortIo).map_err(|_| io_fault)? {
            return Ok(());
        }
        let Some(q) = query(&ec.ports, &mut PortIo).map_err(|_| io_fault)? else { return Ok(()) };
        match ec.queries.get(&q) {
            Some(&m) => {
                if let Err(f) = evaluate(aml, host, m, Vec::new()) {
                    Line::new(b"EC _Q").hex(q as u64).str(b" faulted at ").hex(f.at as u64).end();
                }
            }
            None => Line::new(b"EC query ").hex(q as u64).str(b" has no method").end(),
        }
    }
    Ok(())
}
