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

//! The token and the MAC material the resolver checks on every call.

#[allow(dead_code, clippy::manual_contains, clippy::unnecessary_map_or)]
#[path = "../../../../../src/capabilities/token/types/mod.rs"]
pub mod types;

#[path = "../../../../../src/capabilities/token/material.rs"]
pub mod material;

/// The kernel's MAC check under the bench's key, for `allowlist::for_token`.
pub fn verify_token(t: &types::CapabilityToken) -> bool {
    crate::gate::verify_token(t)
}
