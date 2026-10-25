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

//! A power-button press, the fixed event or a PNP0C0C Notify 0x80, and the
//! hand-over of the fixed button from the kernel's poll to this capsule's
//! SCI, so that one reader alone clears its status bit.

use super::rule::{fixed_button, on_restart, press as verdict, Press};
use super::{power_off, Why};
use crate::sys::acpi_event::button_taken;
use crate::sys::log::Line;
use core::sync::atomic::{AtomicBool, Ordering};
use nonos_aml::events::Policy;

/// The kernel said a capsule served the button before this one.
static RESTARTED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, Default)]
pub struct Presses {
    pub last: Option<u64>,
    /// When the first publish pass finished.
    pub pass_at: Option<u64>,
}

pub fn press(p: &mut Presses, now: u64) {
    let v = on_restart(verdict(p.last, p.pass_at, now), RESTARTED.load(Ordering::Relaxed));
    p.last = Some(now);
    match v {
        // Refused, `serve` gives the shell its Power key instead.
        Press::PowerOff => power_off(Why::Button),
        Press::Bounce => {}
        Press::BootGuard => Line::new(b"power button ignored: within the boot guard").end(),
    }
}

/// The fixed buttons this capsule's SCI serves: the power button, unless
/// the FADT names a control-method one; never the sleep button.
pub fn policy(fadt: &[u8]) -> Policy {
    Policy { power_button: fixed_button(fadt), sleep_button: false }
}

/// Once the SCI is bound: tells the kernel this capsule serves the button.
pub fn take_button(fadt: &[u8]) {
    if !fixed_button(fadt) {
        return;
    }
    match button_taken() {
        Ok(restarted) => {
            RESTARTED.store(restarted, Ordering::Relaxed);
            let again: &[u8] = if restarted { b" again, after a restart" } else { b"" };
            Line::new(b"power button: served here").str(again).end();
        }
        Err(e) => {
            Line::new(b"power button: the kernel kept it, errno ").dec(e.unsigned_abs()).end()
        }
    }
}
