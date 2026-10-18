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

//! After HCRST a controller may hold CNR, and nothing but USBSTS may be
//! written until it clears (xHCI 1.2 section 5.4.2). Bring-up waits for it
//! (`take_over`); so must a revival, or the DCBAA, command ring and event
//! ring it programs are written to a controller not ready to take them. A
//! controller that never gets ready stays dead.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use nonos_devmodel::run;
use nonos_libc::{offer_mmio, take_replies};

use super::bringup_model::{bringup_controller, found, STS, WINDOW};
use super::cnr_device::slow_to_ready;
use crate::constants::USBSTS_HSE;
use crate::protocol::{Request, E_NODEV, OP_ENABLE_SLOT};
use crate::server::{context::Context, mux::Mux};

/// CNR held after each reset while the controller is brought up.
const BRINGUP_MS: u64 = 100;

/// Bring the controller up, fail it, and send a request that revives it,
/// CNR held `revival_ms` after that reset: the reply's status, the driver's
/// generation, and whether a register was written while CNR was set.
fn revived(revival_ms: u64) -> (i32, u32, bool) {
    let bar = bringup_controller();
    let (fault, early) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicBool::new(false)));
    let not_ready = Arc::new(AtomicU64::new(BRINGUP_MS));
    let _hc = run(&bar, slow_to_ready(not_ready.clone(), fault.clone(), early.clone()));
    offer_mmio(bar.base(), WINDOW as u64);
    let driver = crate::setup::run(found()).unwrap_or_else(|e| panic!("bring-up: {e:?}"));
    assert!(!early.load(Ordering::SeqCst), "bring-up waits for CNR");
    let mut mux = Mux::new(vec![Context::new(driver)]);
    not_ready.store(revival_ms, Ordering::SeqCst);
    fault.store(true, Ordering::SeqCst);
    while bar.wrote32(STS) & USBSTS_HSE == 0 {
        std::thread::yield_now();
    }
    let req = Request { op: OP_ENABLE_SLOT, flags: 0, request_id: 1, payload_len: 0 };
    let _ = take_replies();
    mux.dispatch(&req, &[], &mut [0u8; 4096]);
    let f = take_replies().pop().expect("a reply");
    let status = i32::from_le_bytes([f[20], f[21], f[22], f[23]]);
    (status, mux.ctxs[0].driver.generation, early.load(Ordering::SeqCst))
}

#[test]
fn a_revival_writes_nothing_until_the_reset_controller_is_ready() {
    let (_, generation, early) = revived(BRINGUP_MS);
    assert_eq!(generation, 1, "revived");
    assert!(!early, "DCBAAP was written while CNR was set");
}

#[test]
fn a_controller_that_never_gets_ready_after_its_reset_stays_dead() {
    let (status, generation, early) = revived(3_600_000);
    assert_eq!((status, generation), (E_NODEV, 0), "refused, not revived");
    assert!(!early, "DCBAAP was written while CNR was set");
}
