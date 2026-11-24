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

//! The words a personality asks a front for its family's grant with, and the
//! answer, the same on both fronts.
//!
//! - net.socks5 reads a request's first byte as its marker
//!   (capsule_socks5 server/request/marks.rs): GRANT_POLICY is followed by
//!   the policy's bytes, GRANT_BOX by nothing.
//! - net.anon reads an op from its request header (capsule_net_anon
//!   protocol): OP_GRANT_POLICY with the policy's bytes as its body,
//!   OP_GRANT_BOX with none.
//!
//! The answer is one byte: GRANTED, or a refusal's code (refusal.rs).

/// net.socks5's marker for a grant by policy.
pub const SOCKS_GRANT_POLICY: u8 = 7;
/// net.socks5's marker for a grant by box.
pub const SOCKS_GRANT_BOX: u8 = 8;
/// net.anon's op for a grant by policy.
pub const ANON_OP_GRANT_POLICY: u16 = 13;
/// net.anon's op for a grant by box.
pub const ANON_OP_GRANT_BOX: u16 = 14;
/// The answer to a grant made.
pub const GRANTED: u8 = 0;
/// The answer a front gives a sender without Network and without a grant,
/// to any other request: never stream bytes, never a SOCKS reply.
pub const NOT_GRANTED: u8 = 0x47;
