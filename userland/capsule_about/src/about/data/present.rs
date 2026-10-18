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
//! through the first display driver in the kernel's `displays.rs` that has
//! announced itself, and otherwise asks the kernel to blit its frame into
//! the framebuffer the firmware left (compositor `setup/discover.rs`,
//! `setup/prime_gop.rs`). Pure, so the choice is proven on the host.

pub struct Path {
    pub hops: [&'static [u8]; 3],
    pub backend: &'static [u8],
}

/// The path through `display`, the announced display driver's service, or
/// through the firmware framebuffer when none has announced.
pub fn path(display: Option<&'static str>) -> Path {
    if let Some(name) = display {
        let (hop, backend): (&'static [u8], &'static [u8]) = match name {
            "driver.virtio_gpu0" => (b"driver.virtio_gpu", b"compositor + driver.virtio_gpu"),
            "driver.bga0" => (b"driver.bga", b"compositor + driver.bga"),
            _ => (name.as_bytes(), b"compositor + display driver"),
        };
        Path { hops: [b"capsule_about", b"compositor", hop], backend }
    } else {
        Path {
            hops: [b"capsule_about", b"compositor", b"kernel blit"],
            backend: b"compositor + firmware framebuffer",
        }
    }
}
