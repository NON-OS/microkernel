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

//! A grant entered only while the claim it was made under still holds
//! (K10). The check and the entry are one critical section under CLAIMS. A
//! release takes the claim out under CLAIMS before its device's grants are
//! drained, so the drain finds a grant entered first, and a grant entered
//! after finds no claim and is refused: none outlives its claim.

use super::state::CLAIMS;

/// `record` run under CLAIMS when `pid` still holds `device_id` at `epoch`.
pub(crate) fn record_while_held<R>(
    pid: u32,
    device_id: u64,
    epoch: u64,
    record: impl FnOnce() -> R,
) -> Option<R> {
    let claims = CLAIMS.lock();
    let held = claims.iter().any(|c| c.device_id == device_id && c.pid == pid && c.epoch == epoch);
    let entered = held.then(record);
    drop(claims);
    entered
}
