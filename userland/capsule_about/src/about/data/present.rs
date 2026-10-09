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

//! The way this window's pixels reach the screen. The compositor presents
//! through the virtio-gpu driver when that driver has announced
//! `driver.virtio_gpu0`, and otherwise asks the kernel to blit its frame into
//! the framebuffer the firmware left (compositor `setup/discover.rs`,
//! `setup/prime_gop.rs`). Pure, so the choice is proven on the host.

/// The service the compositor looks up for its gfx driver.
pub const GFX_SERVICE: &[u8] = b"driver.virtio_gpu0";

pub struct Path {
    pub hops: [&'static [u8]; 3],
    pub backend: &'static [u8],
}

pub fn path(virtio_announced: bool) -> Path {
    if virtio_announced {
        Path {
            hops: [b"capsule_about", b"compositor", b"driver.virtio_gpu"],
            backend: b"compositor + driver.virtio_gpu",
        }
    } else {
        Path {
            hops: [b"capsule_about", b"compositor", b"kernel blit"],
            backend: b"compositor + firmware framebuffer",
        }
    }
}
