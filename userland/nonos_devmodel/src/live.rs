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

//! A device that answers back.
//!
//! [`FakeBar`](super::FakeBar) alone is memory, and memory reads back whatever
//! was written to it. That is enough to check what a driver leaves behind, and
//! not enough for most of what a bring-up sequence actually does. A reset
//! handshake writes a bit and waits for the device to echo it, then clears it
//! and waits for the device to drop it. A feature negotiation writes the bits
//! it wants and reads back to see which the device kept. Against a passive
//! window the first spins forever and the second can never be refused, so the
//! refusal paths, which are the ones worth proving, are unreachable.
//!
//! The way out is not to intercept the driver's reads. It is to put a second
//! agent on the register file, which is what a device is: the driver polls
//! memory while the device concurrently changes it. [`run`] starts a closure on
//! its own thread and lets it do exactly that. The closure is the device model,
//! written from the specification, and it can echo a bit, self-clear one,
//! refuse a feature, or answer differently on a later read.
//!
//! This is a test harness and it is honest about being one: it proves the
//! driver's protocol against a device that behaves as the specification
//! describes, which is what makes the result portable. It does not model
//! timing, and a driver that only works because a real part is slow will pass
//! here and fail on hardware.
//!
//! # What a concurrent model can and cannot reach
//!
//! It reaches anything the driver **waits** for. A reset handshake that writes
//! a bit, spins until the device echoes it, clears it, and spins until the
//! device drops it is the case this exists for, and it is one a passive window
//! can never satisfy: whichever value you preload, one of the two waits spins
//! forever. Here the model simply answers each edge in turn.
//!
//! It does not reach a register the driver writes and reads back in the next
//! instruction. The virtio feature negotiation is the example: `w8(STATUS,
//! FEATURES_OK)` followed immediately by `r8(STATUS)` leaves a model thread
//! nanoseconds to intervene, and it will usually lose. Modelling a device that
//! reacts *within* a single write-then-read needs the write itself intercepted,
//! which raw volatile stores to memory do not allow, and faking it with a
//! timing-dependent race would produce a test that passes most of the time,
//! which is worse than no test.
//!
//! So the rule for choosing a model: if the driver spins, a live device can
//! answer it. If the driver reads back immediately, it cannot, and the property
//! stays documented rather than asserted.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use super::bar::FakeBar;

/// A device model running against a window. Stopping it joins the thread, so a
/// test that drops this guard has no model still touching the window.
pub struct LiveDevice {
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Drop for LiveDevice {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

/// Run `device` against `bar` until the returned guard is dropped.
///
/// `device` is called in a tight loop and must be cheap and idempotent: it sees
/// the window in whatever state the driver has left it and reacts, which is the
/// same contract a real part works under. It must not assume it observes every
/// intermediate value, because it does not.
pub fn run<F>(bar: &Arc<FakeBar>, device: F) -> LiveDevice
where
    F: Fn(&FakeBar) + Send + 'static,
{
    let stop = Arc::new(AtomicBool::new(false));
    let (b, s) = (Arc::clone(bar), Arc::clone(&stop));
    let handle = thread::spawn(move || {
        while !s.load(Ordering::Acquire) {
            device(&b);
            std::hint::spin_loop();
        }
    });
    LiveDevice { stop, handle: Some(handle) }
}
