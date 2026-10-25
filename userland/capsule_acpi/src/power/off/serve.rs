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

//! The wiped power-off, asked for anywhere and run from the capsule's loop,
//! where the interpreter is at hand (F-111): \_TTS(5), \_PTS(5) and \_S5
//! first, as for any shutdown (`sleep::prepare_sleep`), \_S5's values to
//! the kernel's soft-off, then AdminShutdown. A failed method is said and
//! the shutdown goes on. A refusal is said; a press asked for then falls
//! back to the shell's Power key, as before this capsule took the button.

use crate::boot::Acpi;
use crate::sleep::prepare_sleep;
use crate::sys::acpi_event::{acpi_event, POWER_BUTTON};
use crate::sys::log::Line;
use core::sync::atomic::{AtomicU8, Ordering};
use nonos_libc::mk_admin_shutdown;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Why {
    Button = 1,
    Battery = 2,
    Thermal = 3,
}

static ASKED: AtomicU8 = AtomicU8::new(0);
const SOFT_OFF: u8 = 5;

/// Asks for the power-off; the loop's next `serve` runs it.
pub fn power_off(why: Why) {
    ASKED.store(why as u8, Ordering::Relaxed);
}

fn said(why: u8) -> &'static [u8] {
    match why {
        1 => b"power button",
        2 => b"battery critical",
        _ => b"thermal zone at _CRT",
    }
}

/// The power-off asked for, if any: None when none was, else the errno of
/// the kernel's refusal (it returns only when refused).
pub fn serve(acpi: &mut Acpi) -> Option<i64> {
    let why = ASKED.swap(0, Ordering::Relaxed);
    if why == 0 {
        return None;
    }
    Line::new(b"power off with the wipe: ").str(said(why)).end();
    let _ = prepare_sleep(acpi, Some(SOFT_OFF));
    let e = mk_admin_shutdown();
    Line::new(b"power off refused, errno ").dec(e.unsigned_abs()).end();
    if why == Why::Button as u8 {
        let _ = acpi_event(POWER_BUTTON, 0);
    }
    Some(e)
}
