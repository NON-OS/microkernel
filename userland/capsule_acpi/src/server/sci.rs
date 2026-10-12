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

//! One SCI: the event layer serves it, the notifications the methods queued
//! are said and dropped (the services that act on them come with F 10 and
//! F 11), and the line is acknowledged so the next one can arrive.

use crate::boot::{evaluate, path, Acpi};
use crate::events_io::PortIo;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::events::Dispatch;
use nonos_libc::{mk_irq_ack, mk_irq_poll, IrqPollOut};

/// Whether the SCI fired since the last look.
fn fired(acpi: &mut Acpi) -> bool {
    let Some(sci) = acpi.sci.as_mut() else { return false };
    let mut out = IrqPollOut { seq: 0, overflow: 0 };
    if mk_irq_poll(sci.grant, &mut out) < 0 || out.seq == sci.seq {
        return false;
    }
    sci.seq = out.seq;
    true
}

pub fn serve(acpi: &mut Acpi) {
    if !fired(acpi) {
        return;
    }
    let Acpi { aml, host, events, sci, ec, .. } = acpi;
    if let Some(ev) = events.as_mut() {
        let mut run = |d| match d {
            Dispatch::Method(id) => evaluate(aml, host, id, Vec::new()).map(|_| ()),
            Dispatch::Native(_) => ec.as_ref().map_or(Ok(()), |e| crate::ec::serve(e, aml, host)),
        };
        match ev.on_sci(&mut PortIo, &mut run) {
            Ok(r) => {
                acpi.gpe_runs += r.ran.len() as u64;
                for n in r.disabled {
                    Line::new(b"GPE ")
                        .hex(n as u64)
                        .str(b" left disabled: its method failed")
                        .end();
                }
            }
            Err(_) => Line::new(b"SCI: event registers unreachable").end(),
        }
    }
    acpi.scis += 1;
    while let Some((id, value)) = acpi.aml.rt.notifications.pop_front() {
        if !crate::power::notified(&mut acpi.power, &mut acpi.aml, &mut acpi.host, id, value) {
            Line::new(b"notify ")
                .str(path(&acpi.aml, id).as_bytes())
                .str(b" ")
                .hex(value as u64)
                .end();
        }
    }
    if let Some(s) = sci.as_ref() {
        let _ = mk_irq_ack(s.grant);
    }
}
