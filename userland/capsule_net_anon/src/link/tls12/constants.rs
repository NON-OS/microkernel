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


//! Numbers from RFC 5246, RFC 8422, RFC 7905, RFC 5288, RFC 7627 and
//! RFC 5746, and the limits this client holds a server to.

pub const VERSION: u16 = 0x0303;

pub const SUITE_CHACHA20: u16 = 0xCCA8; // ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256
pub const SUITE_AES256_GCM: u16 = 0xC030; // ECDHE_RSA_WITH_AES_256_GCM_SHA384
/// In preference order. ChaCha20 first: its nonce is not on the wire.
pub const SUITES: [u16; 2] = [SUITE_CHACHA20, SUITE_AES256_GCM];

pub const CT_CHANGE_CIPHER_SPEC: u8 = 20;
pub const CT_ALERT: u8 = 21;
pub const CT_HANDSHAKE: u8 = 22;
pub const CT_APPLICATION: u8 = 23;

pub const HS_HELLO_REQUEST: u8 = 0;
pub const HS_CLIENT_HELLO: u8 = 1;
pub const HS_SERVER_HELLO: u8 = 2;
pub const HS_CERTIFICATE: u8 = 11;
pub const HS_SERVER_KEY_EXCHANGE: u8 = 12;
pub const HS_CERTIFICATE_REQUEST: u8 = 13;
pub const HS_SERVER_HELLO_DONE: u8 = 14;
pub const HS_CLIENT_KEY_EXCHANGE: u8 = 16;
pub const HS_FINISHED: u8 = 20;

pub const EXT_SERVER_NAME: u16 = 0;
pub const EXT_SUPPORTED_GROUPS: u16 = 10;
pub const EXT_EC_POINT_FORMATS: u16 = 11;
pub const EXT_SIGNATURE_ALGORITHMS: u16 = 13;
pub const EXT_EXTENDED_MASTER_SECRET: u16 = 23;
pub const EXT_RENEGOTIATION_INFO: u16 = 0xff01;

pub const GROUP_P256: u16 = 23;
pub const CURVE_TYPE_NAMED: u8 = 3;
pub const POINT_UNCOMPRESSED: u8 = 0;

pub const SIG_RSA_PSS_SHA256: u16 = 0x0804;
pub const SIG_RSA_PSS_SHA384: u16 = 0x0805;
pub const SIG_RSA_PKCS1_SHA256: u16 = 0x0401;
pub const SIG_RSA_PKCS1_SHA384: u16 = 0x0501;
pub const SIGNATURES: [u16; 4] = [SIG_RSA_PSS_SHA256, SIG_RSA_PSS_SHA384, SIG_RSA_PKCS1_SHA256, SIG_RSA_PKCS1_SHA384];

/// RFC 8446 4.1.3: the tail of a TLS 1.3 server's random when it negotiates
/// TLS 1.2.
pub const DOWNGRADE_TLS12: &[u8; 8] = b"DOWNGRD\x01";

/// RFC 5246 6.2: a plaintext fragment is at most 2^14 bytes, and a protected
/// one at most 2^14 + 2048.
pub const PLAINTEXT_MAX: usize = 1 << 14;
pub const CIPHERTEXT_MAX: usize = PLAINTEXT_MAX + 2048;
/// The largest handshake message read. A relay's certificate is well under
/// this; a server that sends more is not one.
pub const MESSAGE_MAX: usize = 1 << 15;
/// The whole server flight, as nonos_tls bounds its own.
pub const FLIGHT_MAX: usize = 128 * 1024;
/// How long the server may go quiet mid-handshake.
pub const QUIET_MS: i64 = 8_000;
/// The AEAD tag both suites append.
pub const TAG: usize = 16;
