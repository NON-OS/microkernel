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

//! ntor strings and widths, from onion_ntor.c in the fork.

/*
 * The four tweaks are PROTOID plus a suffix, and they are HMAC keys, not hash
 * salts. One wrong and the handshake fails the auth check rather than working.
 */
pub const PROTOID: &[u8] = b"ntor-curve25519-sha256-1";
pub const T_MAC: &[u8] = b"ntor-curve25519-sha256-1:mac";
pub const T_KEY: &[u8] = b"ntor-curve25519-sha256-1:key_extract";
pub const T_VERIFY: &[u8] = b"ntor-curve25519-sha256-1:verify";
pub const M_EXPAND: &[u8] = b"ntor-curve25519-sha256-1:key_expand";
pub const SERVER: &[u8] = b"Server";

/// SHA-1 of the relay's RSA identity key, as the consensus `r` line lists it.
pub const IDENTITY_BYTES: usize = 20;

/// router_id || B || X
pub const ONIONSKIN_BYTES: usize = IDENTITY_BYTES + 32 + 32;

/// Y || auth
pub const REPLY_BYTES: usize = 32 + 32;

/// EXP(Y,x) || EXP(B,x) || ID || B || X || Y || PROTOID
pub const SECRET_INPUT_BYTES: usize = 32 + 32 + IDENTITY_BYTES + 32 + 32 + 32 + 24;

/// verify || ID || B || Y || X || PROTOID || "Server"
pub const AUTH_INPUT_BYTES: usize = 32 + IDENTITY_BYTES + 32 + 32 + 32 + 24 + 6;

/// Df || Db || Kf || Kb. or.h calls it CPATH_KEY_MATERIAL_LEN, 20*2 + 16*2.
pub const KEY_MATERIAL_BYTES: usize = 20 + 20 + 16 + 16;
