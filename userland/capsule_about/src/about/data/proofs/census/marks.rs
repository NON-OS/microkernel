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

//! What the census reads: a process table row, a registry entry, and the
//! marks it tells them by.

/// The registry's authority byte for a capsule that ran on a publisher's
/// signature with no proof behind it.
pub const AUTHORITY_PUBLISHER: u8 = 255;

/// The Network capability (abi/caps.toml).
pub const CAP_NETWORK: u64 = 1 << 2;

/// A process this young may be between creation and its registry entry.
pub const STARTING_MS: u64 = 1_000;

/// One process table row, as much of it as the census needs.
#[derive(Clone, Copy)]
pub struct Proc<'a> {
    pub pid: u32,
    pub state: u8,
    pub uptime_ms: u64,
    pub caps: u64,
    pub name: &'a [u8],
}

/// One registry entry.
#[derive(Clone, Copy)]
pub struct Admitted {
    pub pid: u32,
    pub caps: u64,
    pub authority: u8,
}
