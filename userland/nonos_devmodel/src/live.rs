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
//! was written to it. That is enough to check what a driver leaves behind and
//! not enough for a handshake: a reset that writes a bit and waits for the
//! device to drop it spins forever against a passive window, so the refusal
//! paths, the ones worth proving, are unreachable.
//!
//! The way out is not to intercept the driver's reads. It is to put a second
//! agent on the register file, which is what a device is: the driver polls
//! memory while the device concurrently changes it. [`run`] starts a closure
//! on its own thread and lets it do exactly that, written from the
//! specification; it proves the driver against a device that behaves as the
//! specification describes, not against timing or silicon errata. The README says what a concurrent model can and cannot reach, and why a
//! property it cannot reach stays documented rather than asserted.

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use super::bar::FakeBar;

/// Device models running now, and every turn any of them has taken. A host
/// clock shim reads them through [`turns`] so that a driver's deadline cannot
/// expire before the device had its turn: on a loaded builder a spinning
/// driver can hold the core past its whole deadline while the model waits to
/// be scheduled, and that reads as a part that never answered.
static RUNNING: AtomicUsize = AtomicUsize::new(0);
static TURNS: AtomicU64 = AtomicU64::new(0);

/// The turns device models have taken so far, or `None` when none is running.
pub fn turns() -> Option<u64> {
    if RUNNING.load(Ordering::Acquire) == 0 {
        return None;
    }
    Some(TURNS.load(Ordering::Acquire))
}

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
            RUNNING.fetch_sub(1, Ordering::AcqRel);
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
    RUNNING.fetch_add(1, Ordering::AcqRel);
    let handle = thread::spawn(move || {
        while !s.load(Ordering::Acquire) {
            device(&b);
            TURNS.fetch_add(1, Ordering::AcqRel);
            std::hint::spin_loop();
        }
    });
    LiveDevice { stop, handle: Some(handle) }
}
