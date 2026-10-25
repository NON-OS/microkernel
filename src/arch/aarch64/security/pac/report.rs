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

use super::prepare::{ABSENT, NOT_BUILT, NO_ENTROPY};
use super::secret::Source;
use super::state::status;

/// pac-ret's part of the hardening line.
pub fn describe() -> &'static [u8] {
    match Source::from_status(status()) {
        Some(Source::Rndr) => return b"on, keys from RNDR",
        Some(Source::DeviceTreeSeed) => return b"on, keys from the device tree rng-seed",
        None => {}
    }
    match status() {
        ABSENT => b"absent",
        NOT_BUILT => b"off, image not built with pac-ret",
        NO_ENTROPY => b"off, no RNDR and no rng-seed to key it",
        _ => b"off",
    }
}
