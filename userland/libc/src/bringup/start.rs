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

//! A driver's start: discovery's answer, then the bounded bring-up.

use super::policy::decide;
use super::run::bring_up;
use super::say::say_absent;

/// A driver's whole start: leave at once, saying so, when discovery found
/// nothing (`found` is `None`); otherwise bring the device up within the
/// bounded schedule. `Err` carries the code for `mk_exit`, `EXIT_ABSENT` or
/// `EXIT_GAVE_UP`, and by then nothing the driver claimed is still held.
pub fn start_driver<F, T>(
    driver: &[u8],
    found: Option<F>,
    mut attempt: impl FnMut(&F) -> Result<T, &'static str>,
) -> Result<T, i32> {
    decide(found, || say_absent(driver), |device| bring_up(driver, || attempt(&device)))
}
