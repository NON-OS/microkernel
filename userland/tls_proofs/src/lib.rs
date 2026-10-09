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

//! Host proofs for the TLS client.
//!
//! `nonos_tls` carried no tests at all while the browser, the anon transport,
//! the terminal, nym and the wallet all went through it. Its whole source is
//! included here, every module from the real file, so it is measured rather
//! than assumed: against a certificate and a hello shape taken off the live
//! network, the RFC 8448 handshake trace, and a re-signing gateway's chain.
//!
//! Signatures and the X25519 agreement are IPC calls to the crypto capsule.
//! The stand-in `nonos_libc` answers them with the capsule's own handlers, so
//! what is verified here is verified by the pool's code.

extern crate alloc;

include!("modules_cert.rs");
include!("modules_handshake.rs");
include!("modules_record.rs");

/*
 * The names nonos_tls exports at its root, exported here the same way, so a
 * host proof of a caller can name this crate where the caller names nonos_tls.
 */
pub use alert::description_in_record;
pub use alert::name as alert_name;
pub use app_reader::AppReader;
pub use application_plaintext::{application_plaintext, application_plaintext_cached};
pub use application_request::application_request;
pub use cert_dns_match::matches as cert_names_host;
pub use cert_problem::CertProblem;
pub use client_flight::client_flight;
pub use handshake_alert::handshake_alert;
pub use handshake_state::{Answer, HandshakeState, Progress, Refusal, Start};
pub use traffic_keys::TrafficKeys;

/* The stand-in C library, so a caller's proof can read the call counts. */
pub use nonos_libc as shim;

// The wallet's own TLS code names this crate as nonos_tls; the proofs that
// include it by path resolve that name here.
extern crate self as nonos_tls;

pub mod example_ca;
pub mod example_leaf;
pub mod relay_cert;

#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod tests;
