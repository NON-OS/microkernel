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
//! How many of a snapshot's readings came.

use crate::wallet::net::read_snapshot::Snapshot;

/// How many of the nine readings a snapshot holds came.
pub(super) fn came(snap: &Snapshot) -> usize {
    [
        snap.eth_balance.is_some(),
        snap.nonce.is_some(),
        snap.fee.is_some(),
        snap.nox_balance.is_some(),
        snap.usdc_balance.is_some(),
        snap.claimable.is_some(),
        snap.positions.is_some(),
        snap.passes.is_some(),
        snap.stats.is_some(),
    ]
    .iter()
    .filter(|c| **c)
    .count()
}
