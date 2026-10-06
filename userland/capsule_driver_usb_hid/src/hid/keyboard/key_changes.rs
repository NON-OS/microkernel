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

//! Which keys one report presses and releases against the keys held before
//! it. Pure, so the rule is proven on the host.

use super::boot_report::BootReport;
use super::is_error_code::is_error_code;
use super::is_real_key::is_real_key;

/// Calls `emit(usage, pressed)` for each key `report` presses or releases
/// against `held`: presses first, in slot order, then releases, in the order
/// they were held. A usage named in more than one slot is one key, so it
/// makes one event. Returns the slots to hold for the next report.
///
/// A report with an error code in any slot says nothing about which keys
/// are down. It makes no event and the keys held stay held, so a rollover
/// in the middle of fast typing neither lifts the keys still down nor types
/// them again when the next report names them.
pub fn key_changes(held: &[u8; 6], report: &BootReport, mut emit: impl FnMut(u8, bool)) -> [u8; 6] {
    if report.keys.iter().any(|&key| is_error_code(key)) {
        return *held;
    }
    for (slot, &key) in report.keys.iter().enumerate() {
        if is_real_key(key) && !report.keys[..slot].contains(&key) && !held.contains(&key) {
            emit(key, true);
        }
    }
    for (slot, &key) in held.iter().enumerate() {
        if is_real_key(key) && !held[..slot].contains(&key) && !report.keys.contains(&key) {
            emit(key, false);
        }
    }
    report.keys
}
