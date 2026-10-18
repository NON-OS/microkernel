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

//! Slot power, through slot control: one command at a time, and where the
//! port sets command completed, the next waits for it, up to
//! `COMMAND_MS` (the specification's bound as recalled, unverified).

use super::act::line;
use super::find::Bdf;
use super::regs::{power_word, COMMAND_COMPLETED};
use super::slot_rule::CONTROL_BITS;
use super::slot_wire::{SLOT_CONTROL, SLOT_STATUS};
use super::times::COMMAND_MS;
use crate::sys::acpi_slot::{slot_read, slot_write};
use crate::sys::clock::{now_ms, sleep_ms};

pub fn power(f: Bdf, no_cc: bool, on: bool) {
    // The kernel keeps every bit outside CONTROL_BITS as it reads it.
    let Ok(ctrl) = slot_read(f, SLOT_CONTROL) else {
        return line(f).str(b": slot control unreadable").end();
    };
    let _ = slot_write(f, SLOT_CONTROL, power_word(ctrl as u16, on) & CONTROL_BITS);
    let until = now_ms() + COMMAND_MS;
    let mut done = no_cc;
    while !done && now_ms() < until {
        done = slot_read(f, SLOT_STATUS).unwrap_or(0) as u16 & COMMAND_COMPLETED != 0;
        if !done {
            sleep_ms(10);
        }
    }
    if !no_cc {
        let _ = slot_write(f, SLOT_STATUS, COMMAND_COMPLETED);
    }
    let l = line(f).str(if on { b": power on" } else { b": power off" });
    if done {
        l.end()
    } else {
        l.str(b", command never completed").end()
    }
}
