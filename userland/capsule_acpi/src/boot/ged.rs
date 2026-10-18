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

//! A hardware-reduced platform (ACPI 6.5, 4.1), every Arm one among them,
//! has no SCI and no GPE blocks: its events come through Generic Event
//! Devices, each an interrupt that runs _EVT. The kernel grants no such
//! interrupt to this capsule yet, so they are found and said, not served.

use super::eval::evaluator;
use super::report::path;
use super::state::Sci;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use nonos_aml::events::Events;
use nonos_aml::ged::geds;
use nonos_aml::load::Aml;
use nonos_aml::platform::Interrupt;

fn mode(l: Line, i: &Interrupt) -> Line {
    let l = l.str(b" ").hex(u64::from(i.gsiv));
    let l = l.str(if i.edge { b" edge" } else { b" level" });
    let l = l.str(if i.active_low { b" low" } else { b" high" });
    let l = if i.shared { l.str(b" shared") } else { l };
    if i.wake {
        l.str(b" wake")
    } else {
        l
    }
}

/// Its start, in place of the SCI's: nothing to arm and no line to bind.
pub fn reduced(aml: &mut Aml, host: &mut CapsuleHost) -> (Option<Events>, Option<Sci>) {
    let found = geds(aml, &mut evaluator(host));
    Line::new(b"hardware-reduced: no SCI; GEDs ").dec(found.len() as u64).end();
    for g in &found {
        let l = Line::new(b"GED ").str(path(aml, g.id).as_bytes()).str(b":");
        let l = g.interrupts.iter().fold(l, mode);
        let evt = aml.ns.child(g.id, *b"_EVT").is_some();
        let l = l.str(if evt { b"; _EVT" } else { b"; no _EVT" });
        l.str(b"; not served: no GED interrupt grant yet").end();
    }
    (None, None)
}
