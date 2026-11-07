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

//! A login the role table allows, as the host is told of it.

use alloc::vec::Vec;
use nsh_roles::CapSet;

/// A login the role table allows. The host starts the session capsule with
/// at most `caps` and answers `Server::begin`, or `Server::deny`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admission {
    pub user: Vec<u8>,
    pub role: Vec<u8>,
    /// SHA-512 of the user's hybrid public key.
    pub subject: [u8; 64],
    pub caps: CapSet,
    /// The kernel's hard end for the session.
    pub ends_ms: u64,
}
