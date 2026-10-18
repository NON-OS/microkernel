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

//! A controller as a predecessor that died without its `Drop` left it:
//! running, its rings armed at frames since freed. One bring-up attempt on
//! it, each broker call recorded with what the controller showed then.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::{run, FakeBar};
use nonos_libc::{offer_mmio, start_log, take_log, Logged};

use super::bringup_answer::answer_first_command;
use super::bringup_model::{bringup_controller, found, CMD, STS, WINDOW};
use super::bus_master_seen::seen;
use super::devices::present;
use super::erst_fetch::ErstFetch;
use crate::constants::{USBCMD_HCRST, USBCMD_RUN, USBSTS_HCE};
use crate::error::XhciError;

/// The controller left running: conforming once the driver takes it, or
/// never halting. Counts its resets into `resets`, fetches its ERST at the
/// ERSTBA write (`ErstFetch`), and answers the NOOP unless that failed.
fn left_running(bar: &FakeBar, halts: bool, resets: Arc<AtomicU32>) -> impl Fn(&FakeBar) {
    bar.present32(CMD, USBCMD_RUN);
    bar.present32(STS, 0);
    let (answered, erst) = (AtomicBool::new(false), ErstFetch::on_this_thread());
    move |bar: &FakeBar| {
        let cmd = bar.wrote32(CMD);
        if cmd & USBCMD_HCRST != 0 {
            resets.fetch_add(1, Ordering::SeqCst);
        }
        let hce = erst.turn(bar, cmd);
        if halts {
            present(bar, cmd);
            bar.present32(STS, (bar.wrote32(STS) & !USBSTS_HCE) | hce);
        }
        if halts && cmd & USBCMD_RUN != 0 && hce == 0 && !answered.load(Ordering::SeqCst) {
            answered.store(answer_first_command(bar, 1), Ordering::SeqCst);
        }
    }
}

/// One attempt on the controller left running, its registers first set by
/// `shape`. The result and the calls, each with what the controller showed
/// then (`seen`).
pub fn attempt(shape: impl Fn(&FakeBar), halts: bool) -> (Result<(), XhciError>, Vec<Logged>) {
    let bar = bringup_controller();
    shape(&bar);
    let resets = Arc::new(AtomicU32::new(0));
    let _hc = run(&bar, left_running(&bar, halts, resets.clone()));
    offer_mmio(bar.base(), WINDOW as u64);
    let b = bar.clone();
    start_log(move || seen(&b, resets.load(Ordering::SeqCst)));
    let got = crate::setup::run(found()).map(|_| ());
    (got, take_log())
}
