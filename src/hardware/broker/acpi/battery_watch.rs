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

//! A battery at its critical level while discharging (ACPI 6.5, 10.2.2.11,
//! the critical bit of _BST's state): the machine is about to lose power,
//! and an orderly shutdown now is better than losing what is unsaved. The
//! same watch as a critical thermal zone, kept apart so the two never
//! reset each other.

use super::critical_watch::{Action, Watch};

static WATCH: Watch = Watch::new();

pub fn battery_critical(critical: bool, now_ms: u64) -> Action {
    WATCH.step(critical, now_ms)
}

/// The watch on the clock alone (F-108).
pub fn tick(now_ms: u64) -> Action {
    WATCH.tick(now_ms)
}
