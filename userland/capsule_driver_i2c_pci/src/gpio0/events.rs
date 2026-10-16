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
//! The touchpad's line as driver.gpio0 reports it: gpio0 owns the GPIO
//! controller and armed the pin with the firmware's trigger and polarity;
//! this driver drains the events of its own device and says whether the
//! line rang since the last look. gpio0 is looked for again at most once a
//! second while absent, and any refusal leaves the HID driver on its timer.

use core::sync::atomic::{AtomicU64, Ordering};

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_uptime_ms};

use super::wire::{events, rang, request, REPLY_MAX};
use crate::driver::Doorbell;

const SERVICE: &[u8] = b"driver.gpio0";
const CALL_MS: u64 = 50;
const LOOKUP_EVERY_MS: u64 = 1_000;

static PORT: AtomicU64 = AtomicU64::new(0);
static LOOKED: AtomicU64 = AtomicU64::new(0);

/// `Some(rang)` while gpio0 answers for this device, `None` otherwise.
pub fn look(db: &Doorbell) -> Option<bool> {
    let port = port()?;
    let mut reply = [0u8; REPLY_MAX];
    let req = request(db.key);
    let n = mk_ipc_call_timeout(
        port,
        req.as_ptr(),
        req.len(),
        reply.as_mut_ptr(),
        reply.len(),
        CALL_MS,
    );
    if n < 0 {
        // Gone, perhaps restarted under another port: looked for afresh.
        PORT.store(0, Ordering::Relaxed);
        return None;
    }
    let list = events(&reply[..(n as usize).min(REPLY_MAX)])?.ok()?;
    Some(rang(&list, db.uid, db.pin))
}

fn port() -> Option<u64> {
    let known = PORT.load(Ordering::Relaxed);
    if known != 0 {
        return Some(known);
    }
    let (now, last) = (mk_uptime_ms().max(1) as u64, LOOKED.load(Ordering::Relaxed));
    if last != 0 && now.saturating_sub(last) < LOOKUP_EVERY_MS {
        return None;
    }
    LOOKED.store(now, Ordering::Relaxed);
    let (mut port, mut pid) = (0u32, 0u32);
    let r = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    (r >= 0 && port != 0 && pid != 0).then(|| {
        PORT.store(port as u64, Ordering::Relaxed);
        port as u64
    })
}
