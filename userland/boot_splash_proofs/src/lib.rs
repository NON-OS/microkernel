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

//! Host proofs for the boot splash's logo, text layout and status line, on
//! its own source through `#[path]`; `fetch.rs`, which asks the kernel, aside.

extern crate alloc;

#[path = "."]
pub mod logo {
    #[path = "../../capsule_boot_splash/src/logo/bmp.rs"]
    pub mod bmp;
    #[path = "../../capsule_boot_splash/src/logo/place.rs"]
    pub mod place;
    #[path = "../../capsule_boot_splash/src/logo/prepare.rs"]
    pub mod prepare;
    #[path = "../../capsule_boot_splash/src/logo/record.rs"]
    pub mod record;
    #[path = "../../capsule_boot_splash/src/logo/resample.rs"]
    pub mod resample;
    #[path = "../../capsule_boot_splash/src/logo/shown.rs"]
    pub mod shown;
    #[path = "../../capsule_boot_splash/src/logo/turn.rs"]
    pub mod turn;

    pub use prepare::prepare;
    pub use shown::Shown;
}

#[cfg(test)]
#[path = "../../capsule_boot_splash/src/attest_lines.rs"]
pub mod attest_lines;
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
#[allow(dead_code)] // `fill`, the whole-surface wash the proofs do not draw
#[path = "../../capsule_boot_splash/src/vignette.rs"]
pub mod vignette;
#[cfg(test)]
#[path = "../../capsule_boot_splash/src/warning.rs"]
pub mod warning;

#[cfg(test)]
mod build;
#[cfg(test)]
mod tests;
