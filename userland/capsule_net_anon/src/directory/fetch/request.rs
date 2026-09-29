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

//! Building the HTTP request a DirPort answers.

extern crate alloc;

use alloc::vec::Vec;

/*
 * Directory documents are fetched over plain HTTP and are trusted because they
 * are signed, not because the transport was private. That is the protocol's
 * design, and it is why nothing here reaches for TLS: an authority's DirPort
 * does not speak it.
 *
 * The URL space is still "/tor/..." in the fork, unchanged from upstream.
 */
pub const CONSENSUS_PATH: &[u8] = b"/tor/status-vote/current/consensus-microdesc.z";

/// Certificates for one authority, by the SHA-1 of its identity key in hex.
pub fn keys_path(v3ident_hex: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(16 + v3ident_hex.len());
    out.extend_from_slice(b"/tor/keys/fp/");
    out.extend_from_slice(v3ident_hex);
    out
}

/// Microdescriptors, by a list of their base64 digests joined with a dash.
pub fn micro_path(joined: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(20 + joined.len());
    out.extend_from_slice(b"/tor/micro/d/");
    out.extend_from_slice(joined);
    out.extend_from_slice(b".z");
    out
}

/// A GET with the connection closed afterwards, so the body ends at the close
/// and no chunked or keep-alive framing has to be understood.
pub fn get(path: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(64 + path.len());
    out.extend_from_slice(b"GET ");
    out.extend_from_slice(path);
    out.extend_from_slice(b" HTTP/1.0\r\n");
    out.extend_from_slice(b"Accept-Encoding: deflate\r\n");
    out.extend_from_slice(b"Connection: close\r\n\r\n");
    out
}
