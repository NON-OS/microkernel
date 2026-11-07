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

//! The bring-up trace's lines, built only
//! with `bringup-trace`, which only a development image's own copy turns on
//! (tools/nonos-dev-image --bringup-trace). Each is
//! `[ACPI] trace t=<ms since boot> <what>`; without the feature `line` is
//! None, a constant, and every call site folds away.

use crate::sys::log::Line;

pub const ON: bool = cfg!(feature = "bringup-trace");

/// Every EC transaction is said for this long after boot; after it, only
/// those that failed, so the minute's polls do not fill the log.
pub const EVERY_EC_MS: u64 = 120_000;

#[inline(always)]
pub fn line() -> Option<Line> {
    ON.then(|| Line::new(b"trace t=").dec(crate::sys::clock::now_ms()).str(b" "))
}

/// One bring-up step begun.
pub fn step(name: &[u8]) {
    if let Some(l) = line() {
        l.str(b"step ").str(name).end();
    }
}
