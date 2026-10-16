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

//! What a slot's phase asks for, done and said: an arrival probed, a
//! departure or eject taken out of the kernel's table (`functions.rs`),
//! power switched through slot control (`power.rs`).

use super::find::Bdf;
use super::functions::{eject, gone, probe};
use super::power::power;
use super::slot::Act;
use crate::boot::Acpi;
use crate::sys::log::Line;

pub fn line(f: Bdf) -> Line {
    let l = Line::new(b"native hot plug ").dec(f.1.into()).str(b":").dec(f.2.into());
    l.str(b".").dec(f.3.into())
}

fn say(f: Bdf, what: &[u8], n: usize) {
    line(f).str(what).dec(n as u64).str(b" functions").end();
}

/// `counting`: the button's countdown just ran out, so a refusal is new.
pub fn run(acpi: &mut Acpi, i: usize, act: Act, counting: bool) {
    let p = &acpi.hotplug.native.ports[i];
    let (f, cap, no_cc) = (p.bdf, p.cap, p.no_cc);
    match act {
        Act::Nothing => {}
        Act::Probe => say(f, b": arrived, ", probe(f)),
        Act::Depart => say(f, b": left, ", gone(f)),
        Act::Eject => match eject(f) {
            Ok(n) => {
                say(f, b": ejected, ", n);
                let next = acpi.hotplug.native.ports[i].slot.ejected();
                run(acpi, i, next, false);
            }
            Err(()) if counting => line(f).str(b": eject waits, a function is held").end(),
            Err(()) => {}
        },
        Act::PowerOn | Act::PowerOff => power(f, cap, no_cc, act == Act::PowerOn),
    }
}
