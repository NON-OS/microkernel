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
//! The boot framebuffer's holder (boot_fb_rule.rs), read from the table and
//! the claims as list_shown reads them: the table, then each record's claim.

use crate::hardware::broker::claim::lookup;

use super::boot_fb_rule::holds_boot_fb;
use super::state::TABLE;

/// Whether a claimed device's memory window contains `[start, start + len)`.
pub fn boot_fb_held(start: u64, len: u64) -> bool {
    let table = TABLE.read();
    let claimed = table.iter().filter(|r| lookup(r.device_id).is_some());
    // Every slot: an empty one is BarKind::None and contains nothing.
    let windows = claimed.map(|r| r.bars.iter());
    holds_boot_fb((start, len), windows, |b| (b.kind, b.base, b.size))
}
