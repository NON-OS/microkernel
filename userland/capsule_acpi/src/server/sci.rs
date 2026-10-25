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

//! One SCI: the fixed events first, then the event layer serves the GPEs;
//! the notifications the methods queued are routed, and the line is
//! acknowledged so the next one can arrive. A GPE past the storm bound, or
//! the GPE of an EC that has stalled or flooded, is turned off; one left off
//! for a back-off window is armed again first, even with no SCI pending.

use super::sci_parts::{fired, press, rearm};
use crate::boot::{evaluate, Acpi};
use crate::events_io::PortIo;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::events::Dispatch;
use nonos_libc::mk_irq_ack;

pub fn serve(acpi: &mut Acpi) {
    rearm(acpi);
    if !fired(acpi) {
        return;
    }
    // The fixed events first: a pressed power button is acted on before
    // any GPE method or EC query can take time.
    let first = acpi.events.as_mut().map_or(Ok(0), |ev| ev.take_fixed(&mut PortIo));
    press(acpi, first.unwrap_or(0));
    let mut later = 0;
    let Acpi { aml, host, events, ec, .. } = acpi;
    if let Some(ev) = events.as_mut() {
        let mut run = |d| match d {
            Dispatch::Method(id) => evaluate(aml, host, id, Vec::new()).map(|_| ()),
            Dispatch::Native(_) => ec.as_ref().map_or(Ok(()), |e| crate::ec::serve(e, aml, host)),
        };
        match ev.on_sci(&mut PortIo, &mut run) {
            Ok(r) => {
                later = r.fixed;
                acpi.gpe_runs += r.ran.len() as u64;
                super::sci_parts::said(&r);
            }
            Err(_) => Line::new(b"SCI: event registers unreachable").end(),
        }
        let ec_gpe = ec.as_ref().and_then(|e| e.gpe).filter(|g| ev.armed.contains(g));
        if let Some(g) = ec_gpe.filter(|_| !host.ec_health.usable()) {
            let _ = ev.quiet(&mut PortIo, g);
        }
    }
    acpi.scis += 1;
    press(acpi, later);
    let grant = acpi.sci.as_ref().map(|s| s.grant);
    super::notify::route(acpi);
    if let Some(g) = grant {
        let _ = mk_irq_ack(g);
    }
}
