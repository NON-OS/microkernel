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

//! Which block requests the log says: pure, proven in
//! kernel_proofs::block_slow.

/// An answer this slow is said even when it succeeds: well past a disk's
/// own latency, well short of a driver's 30 s budget.
pub(crate) const SLOW_MS: u64 = 5_000;

/// Whether a request is said: it failed, or it took SLOW_MS or longer.
pub(crate) const fn said(status: i32, answered_ms: u64) -> bool {
    status != 0 || answered_ms >= SLOW_MS
}
