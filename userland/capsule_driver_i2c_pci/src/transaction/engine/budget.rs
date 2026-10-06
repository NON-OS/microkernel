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
//! The two sums a transfer is run by: how long it may take, and how many
//! read commands are out whose bytes have not been drained.

use crate::constants::{TRANSFER_BASE_MS, TRANSFER_PER_KIB_MS};
use crate::transaction::TransferRequest;

/// Rounds of the poll loop between two reads of the clock.
pub const CLOCK_EVERY: u32 = 16;

/// Milliseconds a transfer of `bytes` may take before it is a timeout.
pub fn budget_ms(bytes: usize) -> u64 {
    TRANSFER_BASE_MS + (bytes as u64 * TRANSFER_PER_KIB_MS).div_ceil(1024)
}

/// Read commands issued whose bytes have not yet been drained.
pub fn in_flight(req: &TransferRequest<'_>, ci: usize, ri: usize) -> u32 {
    ci.saturating_sub(req.write.len()).saturating_sub(ri) as u32
}
