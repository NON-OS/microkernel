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

//! Who drives a panel's backlight (F-96), pure. A display engine that holds
//! the GPU owns it, and firmware's _BCM must not drive it too, or the two
//! fight over one backlight. So _BCM runs only for a panel whose GPU the
//! broker says is claimed by nobody: the boot framebuffer's panel with no
//! engine yet. A GPU that cannot be named or asked about is taken as driven.

pub fn bcm_allowed(gpu_claimed: Option<bool>) -> bool {
    gpu_claimed == Some(false)
}
