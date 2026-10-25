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

//! The controller as a device model on the whole register window: halted
//! and reset as `host_controller` is, and running its rings when their
//! doorbells are rung. Doorbells read `IDLE` until the driver writes one,
//! so its write of 0 to the command doorbell is seen.

use std::sync::{Arc, Mutex, MutexGuard};

use nonos_devmodel::FakeBar;

use super::hc::{Hc, SLOTS};
use crate::conformance::devices::present;
use crate::conformance::model::{CAP_LEN, DOORBELL};
use crate::constants::{USBCMD, USBSTS, USBSTS_HSE};

pub const IDLE: u32 = u32::MAX;
const CMD: usize = CAP_LEN + USBCMD as usize;
pub(super) const STS: usize = CAP_LEN + USBSTS as usize;

pub fn lock(hc: &Mutex<Hc>) -> MutexGuard<'_, Hc> {
    hc.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// What doorbell `target` was rung with since the last look: the DCI.
fn rung(bar: &FakeBar, target: u8) -> Option<u32> {
    let at = DOORBELL + 4 * target as usize;
    let value = Some(bar.wrote32(at)).filter(|&v| v != IDLE)?;
    bar.present32(at, IDLE);
    Some(value)
}

/// The controller, as a device model on the whole register window.
pub fn controller(hc: Arc<Mutex<Hc>>) -> impl Fn(&FakeBar) {
    move |bar| {
        let mut hc = lock(&hc);
        // One read of USBCMD for both: a reset or start is shown done only
        // once the model has taken it.
        let cmd = bar.wrote32(CMD);
        hc.follow_reset(bar, cmd);
        present(bar, cmd);
        if hc.fault {
            bar.present32(STS, bar.wrote32(STS) | USBSTS_HSE);
        }
        hc.abort(bar);
        if rung(bar, 0).is_some() {
            hc.run_commands(bar);
        }
        for slot in 1..=SLOTS {
            if let Some(dci) = rung(bar, slot) {
                hc.doorbell(bar, slot, dci);
            }
        }
    }
}
