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

//! The `s` and `w` lines: what a relay is trusted for and how much it carries.

use crate::directory::lines::args;
use crate::directory::number::decimal;
use crate::path::Flags;

/// Read the flags off an `s` line.
///
pub fn parse(rest: &[u8]) -> Flags {
    let mut flags = Flags::default();
    for name in args(rest) {
        match name {
            b"Running" => flags.running = true,
            b"Valid" => flags.valid = true,
            b"Fast" => flags.fast = true,
            b"Stable" => flags.stable = true,
            b"Guard" => flags.guard = true,
            b"Exit" => flags.exit = true,
            b"Authority" => flags.authority = true,
            _ => {}
        }
    }
    flags
}

/// Read the consensus weight off a `w` line.
///
pub fn bandwidth(rest: &[u8]) -> u32 {
    for field in args(rest) {
        if let Some(value) = field.strip_prefix(b"Bandwidth=") {
            return match decimal(value) {
                Some(weight) if weight <= u32::MAX as u64 => weight as u32,
                /*
                 * Clamped rather than dropped: an authority publishing a
                 * nonsense weight should cost that relay its share, not make
                 * the whole entry unusable.
                 */
                Some(_) => u32::MAX,
                None => 0,
            };
        }
    }
    0
}
