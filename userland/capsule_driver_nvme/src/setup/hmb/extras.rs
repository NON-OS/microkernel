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

use core::sync::atomic::{AtomicBool, Ordering};

use crate::admin::hmb::extras_allowed;
use crate::log::{emit, Line};

/// Set once an attempt that asked for the extras failed; the bring-up
/// schedule runs every attempt in this one process, so it holds for all.
static EXTRAS_FAILED: AtomicBool = AtomicBool::new(false);

/// Whether this attempt asks for the extras.
pub fn extras() -> bool {
    extras_allowed(EXTRAS_FAILED.load(Ordering::Relaxed))
}

/// An attempt that asked for the extras failed: no later attempt asks.
pub fn extras_failed() {
    if !EXTRAS_FAILED.swap(true, Ordering::Relaxed) {
        let mut line = Line::new();
        line.text(b"attempt failed with host memory buffer and queue count asked; ");
        line.text(b"later attempts ask for neither");
        emit(&mut line);
    }
}
