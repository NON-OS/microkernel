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

//! A kernel draw counted while it writes the framebuffer (L79), so a takeover
//! of the scanout waits the last one out before the GPU's driver is let in,
//! and a holder's first picture is not marked by a draw already under way.

use core::ops::Deref;
use core::sync::atomic::{AtomicU32, Ordering::SeqCst};

use super::held::stopped;
use super::state::{framebuffer_state, KernelFramebuffer};

static DRAWS: AtomicU32 = AtomicU32::new(0);

/// The framebuffer, drawn on for as long as this lives.
pub(crate) struct Drawing(&'static KernelFramebuffer);

impl Deref for Drawing {
    type Target = KernelFramebuffer;
    fn deref(&self) -> &KernelFramebuffer {
        self.0
    }
}

impl Drop for Drawing {
    fn drop(&mut self) {
        // Write-combined stores drained before the count says the draw is
        // done: a full fence orders them on x86 as recalled, not verified.
        core::sync::atomic::fence(SeqCst);
        DRAWS.fetch_sub(1, SeqCst);
    }
}

/// The framebuffer the kernel may draw on, counted until the draw ends:
/// none while a driver holds it or a takeover has stopped it.
pub(crate) fn drawable() -> Option<Drawing> {
    if stopped() {
        return None;
    }
    DRAWS.fetch_add(1, SeqCst);
    // Asked again once counted: a stop set before the count could be read
    // waits for nothing, so this draw must see it and back out.
    let fb = framebuffer_state().filter(|_| !stopped());
    if fb.is_none() {
        DRAWS.fetch_sub(1, SeqCst);
    }
    fb.map(Drawing)
}

/// Every draw counted before now has ended. Spun, answering shootdowns, as
/// a draw runs to its end on its own CPU with nothing it waits for.
pub(crate) fn wait_quiet() {
    while DRAWS.load(SeqCst) != 0 {
        crate::smp::serve_shootdowns();
        core::hint::spin_loop();
    }
}
