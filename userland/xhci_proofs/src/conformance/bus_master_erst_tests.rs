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

//! ERSTBA starts the event ring (xHCI 1.2 section 5.5.2.3.2), and the
//! controller fetches ERST[0] at the write, as qemu-xhci does. Bus Master
//! still off, the fetch master-aborts: QEMU shows Host Controller Error and
//! never answers the NOOP, so a bring-up that found Bus Master off at claim,
//! a retry's or a respawn's, failed. Linux turns it on before ERSTBA too.
//! The model that shows it is `erst_fetch`, checked in `erst_fetch_tests`.

use std::sync::atomic::{AtomicBool, Ordering};

use nonos_devmodel::{run, FakeBar};
use nonos_libc::{offer_mmio, start_log, take_log};

use super::bringup_answer::answer_first_command;
use super::bringup_model::{bringup_controller, found, CMD, ERSTBA, WINDOW};
use super::bus_master_log::first_bus_master_on;
use super::devices::present;
use super::model::wrote64;
use crate::constants::USBCMD_RUN;

#[test]
fn erstba_is_written_only_once_bus_mastering_is_on() {
    // The round-3 review's probe: a plain controller, ERSTBA read at each
    // broker call, so the order shows whatever the model makes of it.
    let bar = bringup_controller();
    let answered = AtomicBool::new(false);
    let _hc = run(&bar, move |bar: &FakeBar| {
        let cmd = bar.wrote32(CMD);
        present(bar, cmd);
        if cmd & USBCMD_RUN != 0 && !answered.load(Ordering::SeqCst) {
            answered.store(answer_first_command(bar, 1), Ordering::SeqCst);
        }
    });
    offer_mmio(bar.base(), WINDOW as u64);
    let b = bar.clone();
    start_log(move || wrote64(&b, ERSTBA));
    let got = crate::setup::run(found()).map(|_| ());
    let erstba = first_bus_master_on(&take_log());
    assert_eq!(got, Ok(()), "brought up");
    assert_eq!(erstba, Some(0), "ERSTBA was written before bus mastering went on");
}
