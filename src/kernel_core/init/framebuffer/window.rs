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

//! Where the boot framebuffer lies in physical memory, kept for the broker,
//! which publishes it as a device a display driver can claim (L31).

use spin::Once;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PhysWindow {
    /// The page the first pixel lies in.
    pub page: u64,
    /// Bytes from `page` to the end of the last row: what the kernel maps.
    pub len: u64,
    /// The first pixel's offset into `page`.
    pub offset: u32,
    /// The loader's pixel format code (the handoff's `pixel_format`).
    pub format: u32,
}

static WINDOW: Once<PhysWindow> = Once::new();

pub(super) fn keep(window: PhysWindow) {
    WINDOW.call_once(|| window);
}

pub(crate) fn phys_window() -> Option<PhysWindow> {
    WINDOW.get().copied()
}
