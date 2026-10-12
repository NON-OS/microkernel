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

//! The backlit outputs (ACPI 6.5, B.6): their _BCL levels, and _BQC's
//! level now where the output has it. Brightness is not changed at start;
//! firmware's choice stands until a key or a request moves it.

use super::state::{Backlight, Output};
use crate::boot::{evaluate, path};
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::{Id, Seg};
use nonos_aml::object::Object;
use nonos_aml::video::{backlights, parse_bcl};

fn call(aml: &mut Aml, host: &mut CapsuleHost, dev: Id, seg: Seg) -> Option<Object> {
    let id = aml.ns.child(dev, seg)?;
    evaluate(aml, host, id, Vec::new()).ok()
}

pub fn current(aml: &mut Aml, host: &mut CapsuleHost, dev: Id) -> Option<u8> {
    match call(aml, host, dev, *b"_BQC")? {
        Object::Integer(v) if v <= 100 => Some(v as u8),
        _ => None,
    }
}

pub fn find(aml: &mut Aml, host: &mut CapsuleHost) -> Backlight {
    let mut b = Backlight::default();
    for id in backlights(aml) {
        let Some(levels) = call(aml, host, id, *b"_BCL").and_then(|o| parse_bcl(&o)) else {
            Line::new(b"backlight ").str(path(aml, id).as_bytes()).str(b": _BCL unusable").end();
            continue;
        };
        let now = current(aml, host, id);
        let l = Line::new(b"backlight ").str(path(aml, id).as_bytes());
        let l = l.str(b" levels=").dec(levels.steps.len() as u64).str(b" now=");
        match now {
            Some(n) => l.dec(u64::from(n)).end(),
            None => l.str(b"unknown").end(),
        }
        b.outputs.push(Output { id, levels, current: now });
    }
    b
}
