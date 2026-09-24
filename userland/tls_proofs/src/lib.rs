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
//! the terminal, nym and the wallet all went through it. The pure parsers are
//! included here from the real source so they are measured rather than assumed,
//! against a certificate and a hello shape taken off the live network.
//!
//! The signature verification itself is not reachable here: it is an IPC call to
//! the crypto capsule, so it belongs to a boot and not to this suite. What is
//! reachable is everything that decides *what* gets handed to it.

#[path = "../../nonos_tls/src/constants.rs"]
pub mod constants;
#[path = "../../nonos_tls/src/der_tlv.rs"]
pub mod der_tlv;
#[path = "../../nonos_tls/src/read.rs"]
pub mod read;

#[path = "../../nonos_tls/src/alert.rs"]
pub mod alert;
#[path = "../../nonos_tls/src/cert_at.rs"]
pub mod cert_at;
#[path = "../../nonos_tls/src/cert_count.rs"]
pub mod cert_count;
#[path = "../../nonos_tls/src/cert_dns_match.rs"]
pub mod cert_dns_match;
#[path = "../../nonos_tls/src/cert_ext.rs"]
pub mod cert_ext;
#[path = "../../nonos_tls/src/cert_ext_entry.rs"]
pub mod cert_ext_entry;
#[path = "../../nonos_tls/src/cert_is_ca.rs"]
pub mod cert_is_ca;
#[path = "../../nonos_tls/src/cert_spki.rs"]
pub mod cert_spki;
#[path = "../../nonos_tls/src/cert_time_value.rs"]
pub mod cert_time_value;
#[path = "../../nonos_tls/src/cert_valid_now.rs"]
pub mod cert_valid_now;
#[cfg(test)]
#[path = "../../nonos_tls/src/handshake_step.rs"]
pub mod handshake_step;
#[path = "../../nonos_tls/src/hello_retry.rs"]
pub mod hello_retry;
#[path = "../../nonos_tls/src/hkdf_label.rs"]
pub mod hkdf_label;
#[path = "../../nonos_tls/src/inner_plain.rs"]
pub mod inner_plain;
#[path = "../../nonos_tls/src/server_hello.rs"]
pub mod server_hello;

pub mod example_ca;
pub mod example_leaf;
pub mod relay_cert;

#[cfg(test)]
mod tests;
