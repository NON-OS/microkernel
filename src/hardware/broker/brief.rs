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

//! How long the shutdown waits for a table another CPU holds (L73). A CPU
//! still running holds the claims or the DMA records only across a config
//! or port access with interrupts off, and lets go within microseconds,
//! answered meanwhile if it waits on a shootdown (L75); one stopped by IPI
//! while holding it never does. A millisecond waits out the first and
//! bounds the cost of the second.

/// Whether to try again: until a millisecond after `start`, from `now_ns`.
pub(crate) fn within_a_millisecond(start: u64) -> bool {
    crate::time::now_ns().saturating_sub(start) < 1_000_000
}
