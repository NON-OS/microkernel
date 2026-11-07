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

//! The record's header: the session, the handshake, the host, the
//! attestation document's hash, the user, the grant and the capabilities
//! the session capsule holds. The chain starts from it.

use nsh_wire::SESSION_ID_LEN;

/// More than the largest header: the fixed fields, the largest
/// enrolment, the longest names and every capability. It is all a host is
/// handed of the record before `Started`.
pub const HEADER_MAX: usize = 16 * 1024;

pub(crate) struct Header<'a> {
    pub session_id: [u8; SESSION_ID_LEN],
    pub start_ms: u64,
    pub th_server_hello: &'a [u8; 64],
    pub host_ed: &'a [u8; 32],
    pub host_ml: &'a [u8; 1952],
    pub enrol: &'a [u8],
    pub doc_hash: &'a [u8; 64],
    pub subject: &'a [u8; 64],
    pub user: &'a [u8],
    pub role: &'a [u8],
    pub grant_hash: &'a [u8; 64],
    pub caps: &'a [&'a [u8]],
}
