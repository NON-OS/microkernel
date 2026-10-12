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

use super::state::{framebuffer_state, KernelFramebuffer};
use core::sync::atomic::{AtomicBool, Ordering};

static HELD: AtomicBool = AtomicBool::new(false);

/// Set when the boot framebuffer is claimed, cleared when it is given back.
pub(crate) fn set_held(held: bool) {
    HELD.store(held, Ordering::Release);
}

/// The framebuffer the kernel may draw on: none while a driver holds it.
pub(crate) fn drawable() -> Option<&'static KernelFramebuffer> {
    if HELD.load(Ordering::Acquire) {
        return None;
    }
    framebuffer_state()
}
