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

//! The display modes the recording may use, and which driver shows each.
//! bga keeps widths a multiple of 8 and fits its video memory; virtio-gpu's
//! host sets any size. Every mode here is at least the compositor's 1024x720
//! canvas floor, so the canvas is the screen: one pixel each, never shrunk.

pub const MODES: [(u32, u32); 4] = [(1024, 768), (1280, 720), (1280, 800), (1920, 1080)];

pub const DRIVERS: [&str; 2] = ["driver.virtio_gpu0", "driver.bga0"];

/// The short name a picture's file carries.
pub fn tag(driver: &str) -> &str {
    driver.trim_start_matches("driver.").trim_end_matches('0')
}
