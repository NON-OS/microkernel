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

//! While a display driver holds the boot framebuffer's claim, the kernel draws
//! nothing on it: the present blit, the heartbeat and the boot log all ask
//! `drawable`, so the driver's picture is never scribbled over (L31). The
//! stop and panic screens still draw, since a halted machine has to say why.
//! Once a GPU driver's claim takes the scanout the console stops for the rest
//! of the boot (L79): stopped while the takeover runs, latched gone as that
//! claim is made ready, and drawing again if the claim fails.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::SeqCst};

static HELD: AtomicBool = AtomicBool::new(false);
static GONE: AtomicBool = AtomicBool::new(false);
/// Takeovers under way, each between its stop and its claim's last step.
static TAKERS: AtomicU32 = AtomicU32::new(0);

/// Set when the boot framebuffer is claimed, cleared when it is given back.
pub(crate) fn set_held(held: bool) {
    HELD.store(held, SeqCst);
}

/// Nothing new may be drawn. Read after a draw is counted, against a stop
/// set before the count is read, so one of the two always sees the other.
/// TAKERS before GONE: a takeover latches GONE before it leaves TAKERS, so a
/// draw that finds TAKERS empty then finds GONE set; read the other way, it
/// could miss both and draw on a scanout a GPU driver holds (L79).
pub(super) fn stopped() -> bool {
    HELD.load(SeqCst) || TAKERS.load(SeqCst) > 0 || GONE.load(SeqCst)
}

/// A takeover begins: no draw starts, and those under way are waited out.
pub(crate) fn stop_drawing() {
    TAKERS.fetch_add(1, SeqCst);
    super::drawing::wait_quiet();
}

/// Latched under the claims' lock as the covering claim is made ready, so a
/// claim of the boot framebuffer after it is refused there. Never cleared.
pub(crate) fn set_gone() {
    GONE.store(true, SeqCst);
}

/// A takeover's claim settled. Made ready, `set_gone` has already latched
/// the stop; refused, the console draws again unless another one runs.
pub(crate) fn end_takeover() {
    TAKERS.fetch_sub(1, SeqCst);
}

/// The scanout was taken by a GPU driver's claim, for the rest of the boot.
pub(crate) fn surrendered() -> bool {
    GONE.load(SeqCst)
}

/// Taken, or being taken: a release now does not give the console back.
pub(crate) fn taken() -> bool {
    TAKERS.load(SeqCst) > 0 || GONE.load(SeqCst)
}
