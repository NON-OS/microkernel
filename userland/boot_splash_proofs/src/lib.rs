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

//! Host proofs for the boot splash's logo path, on the splash's own source
//! through `#[path]`. `fetch.rs` is left out: it is the one file that asks
//! the kernel, and everything it hands on is proven here.

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
mod bmp_tests;
#[cfg(test)]
mod build;
#[cfg(test)]
mod draw_tests;
#[cfg(test)]
mod place_tests;
#[cfg(test)]
mod prepare_tests;
#[cfg(test)]
mod record_tests;
#[cfg(test)]
mod refuse_tests;
#[cfg(test)]
mod turn_tests;
