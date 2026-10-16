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

//! The SCI's small parts: whether it fired, a pressed power button acted
//! on, and what one SCI's report says on the log (each GPE left off, why).

use crate::boot::Acpi;
use crate::sys::clock::now_ms;
use crate::sys::log::Line;
use nonos_aml::events::{SciReport, PWRBTN};
use nonos_libc::{mk_irq_poll, IrqPollOut};

pub fn said(r: &SciReport) {
    for n in &r.disabled {
        Line::new(b"GPE ").hex(u64::from(*n)).str(b" left disabled: its method failed").end();
    }
    for n in &r.stormed {
        let l = Line::new(b"GPE ").hex(u64::from(*n));
        l.str(b" turned off: it fired past the storm bound").end();
    }
}

/// Whether the SCI fired since the last look.
pub fn fired(acpi: &mut Acpi) -> bool {
    let Some(sci) = acpi.sci.as_mut() else { return false };
    let mut out = IrqPollOut { seq: 0, overflow: 0 };
    if mk_irq_poll(sci.grant, &mut out) < 0 || out.seq == sci.seq {
        return false;
    }
    sci.seq = out.seq;
    true
}

pub fn press(acpi: &mut Acpi, fixed: u16) {
    if fixed & PWRBTN != 0 {
        crate::power::off::press(&mut acpi.buttons.presses, now_ms());
    }
}
