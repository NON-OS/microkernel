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

//! When the compositor tells the input router its display size (NIRS op
//! 0x0006, `server/display_notice.rs`). The router places absolute pointer
//! events on that size; told only when it asked, it went on placing them on
//! an old or a made-up one after a mode change for as long as its asks timed
//! out (H-09). So every change is told at once, retried until a notice goes,
//! and told again now and then for a router that started since.

/// How soon a notice that did not go is tried again, and how often a size
/// already told is told again.
pub const RETRY_MS: i64 = 100;
pub const REFRESH_MS: i64 = 5000;

/// Whether `size` must be told now: it is a display (neither side zero), and
/// it differs from the size last told (tried again every RETRY_MS until a
/// notice goes) or REFRESH_MS passed since the last notice.
pub fn due(size: (u32, u32), told: Option<(u32, u32)>, last_ms: i64, now_ms: i64) -> bool {
    let since = now_ms.saturating_sub(last_ms);
    let real = size.0 != 0 && size.1 != 0;
    real && ((told != Some(size) && since >= RETRY_MS) || since >= REFRESH_MS)
}
