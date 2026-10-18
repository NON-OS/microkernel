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

//! A controller whose error a reset clears is brought back: halted, reset,
//! given new rings and started again, every slot it had dropped, and the
//! requests that follow are served. USBSTS reading all ones, a Host System
//! Error or a Host Controller Error each say it failed.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nonos_devmodel::{run, FakeBar};
use nonos_libc::{offer_mmio, take_replies};

use super::bringup_device::{controller_that, Once};
use super::bringup_model::{bringup_controller, found, STS, WINDOW};
use crate::constants::{USBSTS, USBSTS_HCE, USBSTS_HCH, USBSTS_HSE};
use crate::protocol::{Request, E_INVAL, OP_DISABLE_SLOT, OP_PORT_STATUS};
use crate::regs::op::controller_failed;
use crate::server::{context::Context, mux::Mux};

#[test]
fn all_ones_or_either_error_is_a_failed_controller() {
    let bar = FakeBar::new(0x10);
    for (sts, failed) in [(0, false), (USBSTS_HCH | 1 << 3, false), (USBSTS_HSE, true)] {
        bar.present32(USBSTS as usize, sts);
        assert_eq!(controller_failed(bar.base()), failed, "{sts:#x}");
    }
    for sts in [USBSTS_HCE, u32::MAX] {
        bar.present32(USBSTS as usize, sts);
        assert!(controller_failed(bar.base()), "{sts:#x}");
    }
}

fn status(mux: &mut Mux, op: u16, body: &[u8]) -> i32 {
    let req = Request { op, flags: 0, request_id: 1, payload_len: body.len() as u32 };
    let _ = take_replies();
    mux.dispatch(&req, body, &mut [0u8; 4096]);
    let frame = take_replies().pop().expect("a reply");
    i32::from_le_bytes([frame[20], frame[21], frame[22], frame[23]])
}

#[test]
fn a_controller_whose_error_a_reset_clears_is_brought_back() {
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
    assert_eq!(status(&mut mux, OP_PORT_STATUS, &[]), 0);
    assert!(mux.ctxs[0].driver.dead, "seen failed");
    assert_eq!(status(&mut mux, OP_DISABLE_SLOT, &[1]), E_INVAL, "served: no such slot now");
    let d = &mux.ctxs[0].driver;
    assert_eq!((d.dead, d.generation, d.revivals), (false, 1, 1));
}
