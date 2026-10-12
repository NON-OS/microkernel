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

//! The brightness keys, which firmware reports as Notify on the output
//! (ACPI 6.5, B.7): 0x85 cycle, 0x86 up, 0x87 down, 0x88 zero, 0x89 the
//! display turned off. Some firmware notifies the graphics adapter instead
//! of the panel; then every output under it moves.

use super::find::current;
use super::set::set;
use super::state::Backlight;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::video::{change_of, step};

const DISPLAY_OFF: u8 = 0x89;

pub fn notified(
    b: &mut Backlight,
    aml: &mut Aml,
    host: &mut CapsuleHost,
    id: Id,
    value: u8,
) -> bool {
    let parent = |o: Id| aml.ns.get(o).map(|e| e.parent);
    let hit: alloc::vec::Vec<usize> = (0..b.outputs.len())
        .filter(|&i| b.outputs[i].id == id || parent(b.outputs[i].id) == Some(id))
        .collect();
    if hit.is_empty() || !(0x85..=DISPLAY_OFF).contains(&value) {
        return false;
    }
    let Some(change) = change_of(value) else {
        Line::new(b"display off by firmware").end();
        return true;
    };
    for i in hit {
        let o = &mut b.outputs[i];
        let from = current(aml, host, o.id).or(o.current);
        let to = step(&o.levels, from, change);
        match set(o, aml, host, to) {
            Ok(()) => Line::new(b"brightness ").dec(u64::from(to)).str(b"%").end(),
            Err(f) => Line::new(b"brightness: _BCM faulted at ").hex(u64::from(f.at)).end(),
        }
    }
    true
}
