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

//! The EC's event (ACPI 6.5, 12.6): its queries drained (`nonos_aml::ec::
//! drain`, at most 32 a call), then each _Qxx run. An EC that floods past
//! the storm bound, or has stalled, is turned off with one line: its space
//! is refused and the SCI path quiets its GPE.

use super::state::Ec;
use crate::boot::evaluate;
use crate::events_io::PortIo;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::ec::{drain, Off};
use nonos_aml::exec::{Fault, FaultKind};
use nonos_aml::load::Aml;

pub fn serve(ec: &Ec, aml: &mut Aml, host: &mut CapsuleHost) -> Result<(), Fault> {
    if !host.ec_health.usable() {
        return Ok(());
    }
    let r = drain(&ec.ports, &mut PortIo, &mut host.ec_floods);
    if host.ec_health.note(&r) {
        say_off(host.ec_health.off);
    }
    let d = r.map_err(|_| Fault::new(FaultKind::Region, 0, 0))?;
    if d.flooded && host.ec_health.flooded() {
        say_off(host.ec_health.off);
    }
    for q in d.queries {
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

pub fn say_off(why: Option<Off>) {
    let why: &[u8] = match why {
        Some(Off::Stalled) => b"three waits in a row timed out",
        Some(Off::Flooded) => b"it raised more events than the storm bound allows",
        None => return,
    };
    Line::new(b"EC turned off: ").str(why).str(b"; its space is refused, its GPE quieted").end();
}
