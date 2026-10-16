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

//! A revival halts and resets a controller the driver already drives, whose
//! rings are still the driver's own frames, and writes no PCI Command: bus
//! mastering, on since bring-up, is never turned back on before that halt
//! and reset, and the old frames go only after them (`revive_tests`).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nonos_devmodel::run;
use nonos_libc::{offer_mmio, start_log, take_log, Call};

use super::bringup_device::{controller_that, Once};
use super::bringup_model::{bringup_controller, found, STS, WINDOW};
use crate::constants::USBSTS_HSE;
use crate::protocol::{Request, OP_ENABLE_SLOT};
use crate::server::{context::Context, mux::Mux};

#[test]
fn a_revival_leaves_bus_mastering_alone() {
    let (bar, fault) = (bringup_controller(), Arc::new(AtomicBool::new(false)));
    let once = Once::AnswersNoop(1);
    let _hc = run(&bar, controller_that(once, Default::default(), fault.clone()));
    offer_mmio(bar.base(), WINDOW as u64);
    let driver = crate::setup::run(found()).unwrap_or_else(|e| panic!("bring-up: {e:?}"));
    let mut mux = Mux::new(vec![Context::new(driver)]);
    fault.store(true, Ordering::SeqCst);
    while bar.wrote32(STS) & USBSTS_HSE == 0 {
        std::thread::yield_now();
    }
    start_log(|| 0);
    let req = Request { op: OP_ENABLE_SLOT, flags: 0, request_id: 1, payload_len: 0 };
    mux.dispatch(&req, &[], &mut [0u8; 4096]);
    let log = take_log();
    assert_eq!(mux.ctxs[0].driver.generation, 1, "revived");
    let pci: Vec<Call> =
        log.iter().map(|l| l.call).filter(|c| matches!(c, Call::PciCommand(_))).collect();
    assert!(pci.is_empty(), "a revival wrote the PCI Command register: {pci:?}");
}
