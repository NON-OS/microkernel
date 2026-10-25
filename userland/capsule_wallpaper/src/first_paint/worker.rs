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
//! The built-in picture on a worker thread (libc's `WorkerSeat`).
//!
//! Setup used to fill the backing, decode the picture and scale it over a
//! whole screen before the service answered anyone. Under emulation that is
//! over a second, and the desktop shell's first call to the wallpaper went
//! unanswered ("ipc.call unanswered wallpaper", err ETIMEDOUT). Setup now
//! hands the painting here and the service loop answers at once. The worker
//! makes no calls; the answer is whether the picture is on the backing.

use alloc::boxed::Box;

use nonos_libc::{Handoff, Look, WorkerSeat};

use super::job::{paint, Backing};

/// The picture linked in, up before any catalog fetch returns.
pub const EMBEDDED: &[u8] =
    include_bytes!("../../../../nonos-data/wallpapers/special-variant-9.jpg");

static DONE: Handoff<bool> = Handoff::new();
static SEAT: WorkerSeat = WorkerSeat::new();

/// Start the painting of `b`. False when no worker could start: nothing was
/// written and the caller paints it itself.
pub fn send(b: Backing) -> bool {
    if !DONE.claim() {
        return false;
    }
    let arg = Box::into_raw(Box::new(b));
    if SEAT.spawn(work, arg as usize).is_ok() {
        return true;
    }
    // SAFETY: the spawn failed, so no worker took the box; it is still this
    // thread's.
    drop(unsafe { Box::from_raw(arg) });
    DONE.release();
    false
}

/// Whether the worker is done, and if so whether the picture is up.
pub fn poll() -> Look<bool> {
    DONE.poll()
}

fn work(arg: usize) {
    // SAFETY: `arg` is the box `send` leaked for this worker, and the spawn
    // succeeded, so the worker owns it from here.
    let b = *unsafe { Box::from_raw(arg as *mut Backing) };
    DONE.put(paint(b, EMBEDDED));
}
