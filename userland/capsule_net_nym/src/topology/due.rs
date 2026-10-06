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

//! The fetch rule, asked about the list this capsule holds.

use super::clock;
use super::directory::Provenance;
use super::refresh::fetch_due;
use super::store::meta;

/// Whether to fetch the node list now, given the gateways and exits the
/// directory state counts.
pub fn fetch_due_now(gateways: usize, exits: usize) -> bool {
    let fetched_until =
        meta().filter(|m| m.provenance == Provenance::Fetched).map(|m| m.not_after_ms);
    fetch_due(gateways, exits, fetched_until, clock::now_ms().ok())
}
