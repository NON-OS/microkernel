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

//! Link handshake constants, from link_handshake.h and cert-spec.

/// CERTS cell certificate types, from src/trunnel/link_handshake.h.
pub const CERT_ED_ID_SIGN: u8 = 4;
pub const CERT_ED_SIGN_LINK: u8 = 5;

/// cert-spec section 2.1: the only version there is.
pub const CERT_VERSION: u8 = 1;

/// Extension naming the key that signed the certificate. CERTEXT_SIGNED_WITH_KEY
/// in src/trunnel/ed25519_cert.h.
pub const EXT_SIGNED_WITH_KEY: u8 = 4;

pub const SIGNATURE_BYTES: usize = 64;

/*
 * tor_cert_get_checkable_sig signs cert->encoded[..len - 64] with no prefix and
 * no domain separator. A prefix here would fail every certificate on the network.
 */

/// Link protocol versions this capsule will speak. 4 brought the four byte
/// circuit id, 5 added link padding negotiation. The live consensus requires
pub const LINK_VERSIONS: [u16; 2] = [4, 5];

/// Seconds in the hour a certificate expiry is counted in.
pub const SECONDS_PER_HOUR: u64 = 3_600;
