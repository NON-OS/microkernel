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

//! The boot framebuffer on the display wire, proved on a framebuffer in
//! host memory. The `#[path]` includes pull in the shipping source; the
//! requests go through the same server core every VRAM driver uses.

#[path = "../../capsule_driver_bootfb/src/bring_up.rs"]
pub mod bring_up;
#[path = "../../capsule_driver_bootfb/src/copy_in.rs"]
pub mod copy_in;
#[path = "../../capsule_driver_bootfb/src/describe.rs"]
pub mod describe;
#[path = "../../capsule_driver_bootfb/src/display.rs"]
pub mod display;
#[path = "../../capsule_driver_bootfb/src/fb.rs"]
pub mod fb;
#[path = "../../capsule_driver_bootfb/src/fb_wipe.rs"]
pub mod fb_wipe;
#[path = "../../capsule_driver_bootfb/src/firmware.rs"]
pub mod firmware;
#[path = "../../capsule_driver_bootfb/src/geometry.rs"]
pub mod geometry;
#[path = "../../capsule_driver_bootfb/src/loss.rs"]
pub mod loss;
#[path = "../../capsule_driver_bootfb/src/record.rs"]
pub mod record;

#[cfg(test)]
mod pool;
#[cfg(test)]
mod tests;
