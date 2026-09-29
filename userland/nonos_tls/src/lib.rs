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

//! A TLS 1.3 client.
//!
//! Lifted out of the browser, which had one copy, while the wallet had
//! another that had already drifted from it in twenty one files. A
//! certificate check that only half the system gets is worse than no shared
//! code at all, so there is one of these now.
//!
//! Hashing, the key schedule and the record ciphers run here, next to the
//! keys they use. The X25519 agreement and every signature check go to the
//! pool service.

#![no_std]

extern crate alloc;

/*
 * Every module sits at the crate root, where the others name it, and the
 * list is long enough to be kept in two files: the certificate side and the
 * handshake and record side.
 */
include!("modules_cert.rs");
include!("modules_handshake.rs");

/// The standard's name for an alert description, so a log can say
/// "handshake_failure" rather than a number nobody looks up.
pub use alert::description_in_record;
pub use alert::name as alert_name;
pub use app_reader::AppReader;
pub use application_plaintext::{application_plaintext, application_plaintext_cached};
pub use application_request::application_request;
pub use application_write::application_write;
/// Exported so the browser's own chain walk enforces the same issuer rule from
/// the same code. A second copy is how one of them gets fixed and the other
/// does not.
pub use cert_is_ca::cert_is_ca;
pub use client_flight::client_flight;
pub use handshake_alert::handshake_alert;
pub use handshake_fault::handshake_fault;
pub use handshake_state::{Answer, HandshakeState, Progress, Refusal, Start};
pub use rtc_now::rtc_now;
pub use server_complete::{server_complete, server_complete_unauthenticated, ServerComplete};
pub use session::{exchange, Io, SessionError};
pub use traffic_keys::TrafficKeys;
