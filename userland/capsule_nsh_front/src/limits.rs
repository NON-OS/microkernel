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

//! The time bounds nsh.front holds a connection to. nsh.front serves one
//! connection at a time, so the handshake bound is what keeps a peer that
//! connects and says nothing from holding every other login out.

/// How long a connection has to finish the handshake, from accept.
pub const HANDSHAKE_MS: u64 = 30_000;
/// The longest nsh.front waits for nsh.attest's host keys, a signature or
/// the machine's records: the first ask of a boot may make and seal the
/// keys and read the store, which under TCG takes longer than the kernel's
/// five-second default. Well inside the handshake bound, so a signature
/// that never comes still lets the handshake end on time.
pub const RELAY_MS: u64 = 15_000;
/// The idle timeout `AUTH_OK` states, in seconds.
pub const IDLE_S: u32 = 600;
/// The most bytes one session's record may hold.
pub const MAX_RECORD: usize = 1 << 20;
/// How long the shell has to exit, and the record to go out, once the server
/// has stopped a session.
pub const STOP_GRACE_MS: u64 = 60_000;
/// The longest one connection may last: the handshake, then the longest
/// session a role table may give (nsh_roles::MAX_SPAN_MS, seven days),
/// then the stop's grace. The server ends every session within the first
/// two, with a reason and the signed record, so this bound only lets go of
/// a connection whose shell never answered the stop.
pub const SESSION_MAX_MS: u64 = HANDSHAKE_MS + nsh_roles::MAX_SPAN_MS + STOP_GRACE_MS;
