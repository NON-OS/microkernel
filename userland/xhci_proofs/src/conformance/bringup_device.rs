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

//! The controller `setup::run` brings up, and the ways it can fail once
//! running. Each counts its resets, so a test can say the controller was
//! reset before it was let go.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;

use super::bringup_answer::answer_first_command;
use super::bringup_model::{CMD, STS};
pub use super::bringup_once::Once;
use super::erst_fetch::ErstFetch;
use crate::constants::{USBCMD_HCRST, USBCMD_RUN, USBSTS_HCH, USBSTS_HSE};

/// The controller, counting its HCRSTs into `resets`, with a Host System
/// Error while `fault` is set; a reset clears it, as HCRST resets USBSTS,
/// and lets the next NOOP be answered. It fetches its ERST at the ERSTBA
/// write (`ErstFetch`). USBSTS is the model's alone: what the driver
/// writes there is RW1C, an acknowledgement, never a state.
pub fn controller_that(
    once: Once,
    resets: Arc<AtomicU32>,
    fault: Arc<AtomicBool>,
) -> impl Fn(&FakeBar) {
    let (ran, answered) = (AtomicU32::new(0), AtomicU32::new(0));
    let erst = ErstFetch::on_this_thread();
    move |bar| {
        let cmd = bar.wrote32(CMD);
        let hce = erst.turn(bar, cmd);
        if cmd & USBCMD_HCRST != 0 {
            resets.fetch_add(1, Ordering::SeqCst);
            fault.store(false, Ordering::SeqCst);
            answered.store(0, Ordering::SeqCst);
            bar.present32(CMD, cmd & !USBCMD_HCRST);
        }
        if cmd & USBCMD_RUN != 0 {
            ran.store(1, Ordering::SeqCst);
        }
        let running = match once {
            Once::Conforming => cmd & USBCMD_RUN != 0,
            Once::AnswersNoop(code) => {
                if cmd & USBCMD_RUN != 0 && hce == 0 && answered.load(Ordering::SeqCst) == 0 {
                    answered.store(answer_first_command(bar, code) as u32, Ordering::SeqCst);
                }
                cmd & USBCMD_RUN != 0
            }
            Once::NeverHalts => ran.load(Ordering::SeqCst) != 0,
        };
        let failed = if fault.load(Ordering::SeqCst) { USBSTS_HSE } else { 0 };
        bar.present32(STS, failed | hce | if running { 0 } else { USBSTS_HCH });
    }
}
