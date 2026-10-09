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

//! The simple 1-RTT handshake of RFC 8448 section 3, byte for byte.
//!
//! The server's key share, certificate and signature are the trace's; the
//! client's X25519 private key is given, so both ends' secrets can be derived
//! and every record opened and compared against the published bytes. The
//! suite is TLS_AES_128_GCM_SHA256.

/// The ClientHello handshake message, header included.
pub const CLIENT_HELLO: &[u8] = include_bytes!("rfc8448/client_hello.tls");
/// The client's X25519 private key.
pub const CLIENT_PRIVATE: &[u8] = include_bytes!("rfc8448/client_private.tls");
/// The ServerHello record, in the clear.
pub const SERVER_HELLO_RECORD: &[u8] = include_bytes!("rfc8448/server_hello_record.tls");
/// EncryptedExtensions through Finished, as one encrypted record.
pub const SERVER_FLIGHT_RECORD: &[u8] = include_bytes!("rfc8448/server_flight_record.tls");
/// The same four messages decrypted, without the content type byte.
pub const SERVER_MESSAGES: &[u8] = include_bytes!("rfc8448/server_messages.tls");
/// The client Finished record.
pub const CLIENT_FINISHED_RECORD: &[u8] = include_bytes!("rfc8448/client_finished_record.tls");
/// Fifty application bytes, 0x00 to 0x31, from the client at sequence zero.
pub const CLIENT_DATA_RECORD: &[u8] = include_bytes!("rfc8448/client_data_record.tls");
/// The same fifty bytes from the server, after its ticket.
pub const SERVER_DATA_RECORD: &[u8] = include_bytes!("rfc8448/server_data_record.tls");
/// The NewSessionTicket the server sends first under its application keys.
pub const SERVER_TICKET_RECORD: &[u8] = include_bytes!("rfc8448/server_ticket_record.tls");
/// The server's close_notify, under its application keys.
pub const SERVER_ALERT_RECORD: &[u8] = include_bytes!("rfc8448/server_alert_record.tls");
