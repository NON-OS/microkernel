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
use super::regs::{power_word, COMMAND_COMPLETED, SLOT_CTRL, SLOT_STATUS};
use super::times::COMMAND_MS;
use super::walk::read;
use crate::sys::acpi_pci::pci_write;
use crate::sys::clock::{now_ms, sleep_ms};

pub fn power(f: Bdf, cap: u16, no_cc: bool, on: bool) {
    let ctrl = read(f, cap + SLOT_CTRL, 2) as u16;
    let _ = pci_write(f, cap + SLOT_CTRL, 2, u32::from(power_word(ctrl, on)));
    let until = now_ms() + COMMAND_MS;
    let mut done = no_cc;
    while !done && now_ms() < until {
        done = read(f, cap + SLOT_STATUS, 2) as u16 & COMMAND_COMPLETED != 0;
        if !done {
            sleep_ms(10);
        }
    }
    if !no_cc {
        let _ = pci_write(f, cap + SLOT_STATUS, 2, u32::from(COMMAND_COMPLETED));
    }
    let l = line(f).str(if on { b": power on" } else { b": power off" });
    if done {
        l.end()
    } else {
        l.str(b", command never completed").end()
    }
}
