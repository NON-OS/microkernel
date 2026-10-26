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

//! Host proofs for the boot splash's canvas, attestation panel, layout and
//! status line, on its own source through `#[path]`; what asks the kernel
//! or the compositor aside.

extern crate alloc;

#[cfg(test)]
#[path = "../../capsule_boot_splash/src/attest_lines.rs"]
pub mod attest_lines;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/canvas.rs"]
pub mod canvas;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/chrome.rs"]
pub mod chrome;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/follow.rs"]
pub mod follow;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/layout.rs"]
pub mod layout;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/layout_fit.rs"]
pub mod layout_fit;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/status.rs"]
pub mod status;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/title.rs"]
pub mod title;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/vignette.rs"]
pub mod vignette;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/warning.rs"]
pub mod warning;

#[cfg(test)]
mod tests;
