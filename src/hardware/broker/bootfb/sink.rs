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

//! The frame a taken-back holder's framebuffer pages show instead (L79): one
//! zeroed page of RAM, made before the framebuffer is published and never
//! freed. Only pages in the device window ever map it, and those are never
//! freed with the tables holding them; only bootfb's own service, the one
//! claimant the framebuffer's hold lets in, ever writes it.

use spin::Once;

use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc;

static SINK: Once<PhysAddr> = Once::new();

/// False when no frame could be had: the framebuffer is not then published.
pub(super) fn make() -> bool {
    let Some(frame) = frame_alloc::allocate_frame() else {
        return false;
    };
    frame_alloc::zero_frame(frame);
    SINK.call_once(|| frame);
    true
}

pub(in crate::hardware::broker) fn get() -> Option<PhysAddr> {
    SINK.get().copied()
}
