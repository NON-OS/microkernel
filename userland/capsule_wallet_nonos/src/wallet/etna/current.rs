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

//! The layout of the window being painted, set once a paint by the frame and
//! read by everything the frame and the type draw.

use core::sync::atomic::{AtomicU32, Ordering};

use super::layout::{layout, Layout, PHOTO_W};

static WIDTH: AtomicU32 = AtomicU32::new(PHOTO_W);

/// Take the window's width for this paint.
pub fn set(width: u32) {
    WIDTH.store(width, Ordering::Relaxed);
}

/// The layout of the window last painted.
pub fn now() -> Layout {
    layout(WIDTH.load(Ordering::Relaxed))
}
