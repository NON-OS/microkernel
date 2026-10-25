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

//! Whether an install may reach its mirror now, through the network the
//! person chose. Pure, so the host proofs hold it.

/// Why an install cannot start now, or None when it may: a mirror on the
/// local network is dialled directly whatever the route, and any other is
/// reached only while the network the person chose is running. `down`
/// asks that network for its reason for being down, when it is; it waits up
/// to three minutes for it to come up, so a local mirror never asks.
pub fn offline(local: bool, down: impl FnOnce() -> Option<&'static str>) -> Option<&'static str> {
    match local {
        true => None,
        false => down(),
    }
}
