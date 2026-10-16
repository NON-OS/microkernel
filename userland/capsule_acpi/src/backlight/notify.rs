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
//! (ACPI 6.5, B.7; recalled, unverified): 0x86 up and 0x87 down go to the
//! shell as keys (F-96), which asks the display engine that owns the panel;
//! 0x85 cycle, 0x88 zero and 0x89 display off are logged and left. Some
//! firmware notifies the graphics adapter instead of the panel.

use super::state::Backlight;
use crate::sys::acpi_event::{acpi_event, AGAIN, BRIGHTNESS};
use crate::sys::clock::sleep_ms;
use crate::sys::log::Line;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;

const UP: u8 = 0x86;
const DOWN: u8 = 0x87;
const TRIES: u32 = 5;
const RETRY_MS: u64 = 20;

fn key(up: bool) {
    for _ in 0..TRIES {
        match acpi_event(BRIGHTNESS, u64::from(up)) {
            Err(AGAIN) => sleep_ms(RETRY_MS),
            _ => return,
        }
    }
    Line::new(b"brightness key: the desktop's input was busy; key dropped").end();
}

pub fn notified(b: &Backlight, aml: &Aml, id: Id, value: u8) -> bool {
    let parent = |o: Id| aml.ns.get(o).map(|e| e.parent);
    let ours = b.outputs.iter().any(|o| o.id == id || parent(o.id) == Some(id));
    if !ours || !(0x85..=0x89).contains(&value) {
        return false;
    }
    match value {
        UP | DOWN => key(value == UP),
        v => Line::new(b"display notify ").hex(u64::from(v)).str(b", left to the engine").end(),
    }
    true
}
