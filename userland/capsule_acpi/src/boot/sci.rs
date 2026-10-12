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

//! The SCI: the kernel's PNP0C08 record claimed and its line bound, the
//! FADT's event ports granted, ACPI mode entered, the GPE methods armed.

use super::eval::evaluator;
use super::sci_claim::{claim_and_bind, grant_ports};
use super::state::Sci;
use crate::events_io::PortIo;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use nonos_aml::events::{enter_acpi_mode, handlers, parse_fadt, wake_gpes, Events, Pm1Cnt, Policy};
use nonos_aml::load::Aml;

/// How long firmware gets to answer ACPI_ENABLE.
const MODE_WAIT_MS: u64 = 3000;

pub fn start(fadt: &[u8], aml: &mut Aml, host: &mut CapsuleHost) -> (Option<Events>, Option<Sci>) {
    let f = match parse_fadt(fadt) {
        Ok(f) if !f.hw_reduced => f,
        _ => {
            Line::new(b"no fixed event hardware; no SCI").end();
            return (None, None);
        }
    };
    if !grant_ports(&f) {
        Line::new(b"event ports refused").end();
        return (None, None);
    }
    host.pm1_cnt = f.pm1a_cnt.map(|a| Pm1Cnt { a, b: f.pm1b_cnt });
    match enter_acpi_mode(&f, &mut PortIo, MODE_WAIT_MS) {
        Ok(switched) => {
            Line::new(if switched { b"entered ACPI mode" } else { b"already in ACPI mode" }).end()
        }
        Err(e) => {
            Line::new(b"ACPI mode not entered: ").str(alloc::format!("{e:?}").as_bytes()).end()
        }
    }
    let (found, both) = handlers(&aml.ns);
    let (wake, bad) =
        wake_gpes(aml, &mut |aml, id| evaluator(host)(aml, id, alloc::vec::Vec::new()));
    let events = Events::new(&f, found, &wake, Policy::default())
        .ok()
        .filter(|e| e.start(&mut PortIo).is_ok());
    let armed = events.as_ref().map_or(0, |e| e.armed.len());
    Line::new(b"GPEs armed ")
        .dec(armed as u64)
        .str(b", wake ")
        .dec(wake.len() as u64)
        .str(b", both _L and _E ")
        .dec(both.len() as u64)
        .str(b", _PRW failed ")
        .dec(bad.len() as u64)
        .end();
    let sci = events.as_ref().and_then(|_| claim_and_bind());
    if sci.is_none() {
        Line::new(b"SCI not bound; events are not served").end();
    }
    (events, sci)
}
