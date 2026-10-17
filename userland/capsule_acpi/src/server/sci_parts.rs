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
use crate::events_io::PortIo;
use crate::sys::clock::now_ms;
use crate::sys::log::Line;
use nonos_aml::events::{SciReport, PWRBTN};
use nonos_libc::{mk_irq_poll, IrqPollOut};

pub fn said(r: &SciReport) {
    for (n, off) in &r.disabled {
        let l = Line::new(b"GPE ").hex(u64::from(*n));
        match off {
            None => l.str(b" left disabled: its method failed").end(),
            Some(ms) => l.str(b" off for ").dec(*ms).str(b" ms: its runs were cut").end(),
        }
    }
    for n in &r.stormed {
        let l = Line::new(b"GPE ").hex(u64::from(*n));
        l.str(b" turned off: it fired past the storm bound").end();
    }
}

/// Arms again each GPE whose back-off window has passed; a level source
/// still asserting is then served on the next SCI.
pub fn rearm(acpi: &mut Acpi) {
    let Some(ev) = acpi.events.as_mut() else { return };
    match ev.rearm_due(&mut PortIo) {
        Ok(back) => back.iter().for_each(|n| {
            Line::new(b"GPE ").hex(u64::from(*n)).str(b" armed again after its window").end()
        }),
        Err(_) => Line::new(b"SCI: event registers unreachable").end(),
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
