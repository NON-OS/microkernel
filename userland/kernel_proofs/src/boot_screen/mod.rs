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

/*
 * The panic and stop screens draw through the loader's identity mapping of
 * the firmware framebuffer before the kernel maps its own. The kernel's
 * check of the frame is included by path, and so are the loader's own
 * files that decide what it maps, so the two cannot drift apart.
 */

#[allow(dead_code)]
#[path = "../../../../src/sys/boot_log/early_frame.rs"]
pub mod early_frame;

#[allow(dead_code)]
#[path = "../../../../nonos-bootloader/src/paging/constants.rs"]
pub mod constants;
#[allow(dead_code)]
#[path = "../../../../nonos-bootloader/src/paging/fb_window.rs"]
pub mod fb_window;

#[cfg(test)]
mod tests;
