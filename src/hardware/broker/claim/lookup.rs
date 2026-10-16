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

use super::state::CLAIMS;
use super::types::Claim;

/// The device's claim once it is made. One still being made reads as none,
/// so nothing is mapped, bound or written for a claim the broker may yet
/// refuse (L65).
pub fn lookup(device_id: u64) -> Option<Claim> {
    CLAIMS.lock().iter().find(|c| c.device_id == device_id && c.ready).copied()
}
