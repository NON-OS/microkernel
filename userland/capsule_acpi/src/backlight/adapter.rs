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

//! The graphics adapters above the backlit outputs (ACPI 6.5, B.4): each
//! is told, with _DOS(0), that firmware keeps both output switching and
//! the brightness change on AC and battery, NONOS doing neither; and its
//! _DOD output IDs are logged, against which each output's _ADR is the
//! panel's identity on the brightness wire.

use super::state::Backlight;
use crate::boot::{evaluate, path};
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::object::Object;
use nonos_aml::video::parse_dod;

const FIRMWARE_DECIDES: u64 = 0;

pub fn adapters(b: &Backlight, aml: &mut Aml, host: &mut CapsuleHost) {
    let mut done = Vec::new();
    for o in &b.outputs {
        let Some(gfx) = aml.ns.get(o.id).map(|e| e.parent) else { continue };
        if done.contains(&gfx) || aml.ns.child(gfx, *b"_DOS").is_none() {
            continue;
        }
        done.push(gfx);
        if let Some(dos) = aml.ns.child(gfx, *b"_DOS") {
            let _ = evaluate(aml, host, dos, vec![Object::Integer(FIRMWARE_DECIDES)]);
        }
        let ids = aml
            .ns
            .child(gfx, *b"_DOD")
            .and_then(|d| evaluate(aml, host, d, Vec::new()).ok())
            .map_or_else(Vec::new, |o| parse_dod(&o));
        let mut l = Line::new(b"display adapter ").str(path(aml, gfx).as_bytes()).str(b" outputs");
        for id in ids.iter().take(8) {
            l = l.str(b" ").hex(u64::from(*id));
        }
        l.end();
    }
}
