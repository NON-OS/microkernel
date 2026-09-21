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

//! What a directory authority is, and the quorum over them.

pub struct Authority {
    pub address: [u8; 4],
    pub dir_port: u16,
    /// SHA-1 of the v3 identity key. The trust anchor: a certificate whose
    /// identity key does not hash to this is not this authority's, whoever
    pub v3ident: [u8; 20],
}

/// Four of seven. A client accepting one signature follows any single authority
/// that is compromised or merely wrong.
pub const REQUIRED_SIGNATURES: usize = super::list::AUTHORITIES.len() / 2 + 1;
