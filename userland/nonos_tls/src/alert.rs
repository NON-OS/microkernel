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

//! The record a peer sends to say why it is stopping.

/// Alert content type, RFC 8446 section 5.1.
pub const ALERT: u8 = 21;

/*
 * Two bytes, a level and a description, in the clear before the handshake
 * keys exist and inside a protected record after.
 */
pub fn description_in_record(buf: &[u8]) -> Option<u8> {
    if buf.len() < 5 || buf[0] != ALERT {
        return None;
    }
    let len = u16::from_be_bytes([buf[3], buf[4]]) as usize;
    // Exactly two bytes, and both of them present.
    if len != 2 || buf.len() < 7 {
        return None;
    }
    Some(buf[6])
}

/// The alert description inside an already decrypted record, given the inner
/// content type and the bytes before it.
pub fn description_in_plaintext(inner_type: u8, content: &[u8]) -> Option<u8> {
    if inner_type != ALERT || content.len() != 2 {
        return None;
    }
    Some(content[1])
}

// The ones a client can act on. close_notify is an ending, not a fault.
pub fn name(description: u8) -> &'static str {
    match description {
        0 => "close_notify",
        40 => "handshake_failure",
        42 => "bad_certificate",
        45 => "certificate_expired",
        47 => "illegal_parameter",
        48 => "unknown_ca",
        50 => "decode_error",
        51 => "decrypt_error",
        70 => "protocol_version",
        71 => "insufficient_security",
        80 => "internal_error",
        112 => "unrecognized_name",
        116 => "certificate_required",
        120 => "no_application_protocol",
        _ => "alert",
    }
}
