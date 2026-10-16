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

//! A controller that holds CNR for a while after each HCRST, as silicon
//! may, and notes whether DCBAAP was written meanwhile: no operational
//! register but USBSTS may be written until CNR reads 0 (xHCI 1.2 section
//! 5.4.2). CNR is shown before the reset is, so a driver that waits for it
//! never writes early.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use nonos_devmodel::FakeBar;

use super::bringup_answer::answer_first_command;
use super::bringup_model::{CMD, STS};
use super::erst_fetch::ErstFetch;
use super::model::{wrote64, CAP_LEN};
use crate::constants::{DCBAAP_LO, USBCMD_HCRST, USBCMD_RUN, USBSTS_CNR, USBSTS_HCH, USBSTS_HSE};

/// The controller, CNR held `not_ready_ms` after each reset, a Host
/// System Error while `fault` is set (a reset clears it), and `early` set
/// once DCBAAP changed while CNR was set. It fetches its ERST at the ERSTBA
/// write (`ErstFetch`).
pub fn slow_to_ready(
    not_ready_ms: Arc<AtomicU64>,
    fault: Arc<AtomicBool>,
    early: Arc<AtomicBool>,
) -> impl Fn(&FakeBar) {
    let (ready_at, answered) = (Mutex::new(Instant::now()), AtomicU32::new(0));
    let (dcbaap, erst) = (Mutex::new(0u64), ErstFetch::on_this_thread());
    move |bar| {
        let cmd = bar.wrote32(CMD);
        let hce = erst.turn(bar, cmd);
        let reset = cmd & USBCMD_HCRST != 0;
        if reset {
            fault.store(false, Ordering::SeqCst);
            answered.store(0, Ordering::SeqCst);
            let not_ready = Duration::from_millis(not_ready_ms.load(Ordering::SeqCst));
            *ready_at.lock().unwrap() = Instant::now() + not_ready;
        }
        let cnr = Instant::now() < *ready_at.lock().unwrap();
        let running = cmd & USBCMD_RUN != 0;
        if running && hce == 0 && answered.load(Ordering::SeqCst) == 0 {
            answered.store(answer_first_command(bar, 1) as u32, Ordering::SeqCst);
        }
        let failed = if fault.load(Ordering::SeqCst) { USBSTS_HSE } else { 0 };
        let halted = if running { 0 } else { USBSTS_HCH };
        bar.present32(STS, failed | hce | halted | if cnr { USBSTS_CNR } else { 0 });
        if reset {
            bar.present32(CMD, cmd & !USBCMD_HCRST);
        }
        let now = wrote64(bar, CAP_LEN + DCBAAP_LO as usize);
        let mut last = dcbaap.lock().unwrap();
        if now != *last {
            early.fetch_or(cnr, Ordering::SeqCst);
            *last = now;
        }
    }
}
