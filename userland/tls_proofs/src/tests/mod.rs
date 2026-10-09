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

//! The proofs themselves, kept out of the crate root so the list of
//! production files included above stays readable.

mod alert_named;
mod alert_plaintext;
mod answer_gateway;
mod answer_refused;
mod answer_cert_request;
mod answer_rfc8448;
mod app_reader_page;
mod app_reader_splits;
mod cert_message;
mod cert_request;
mod cert_problem;
mod cert_problem_flight;
mod cert_message_refusal;
mod cert_message_vectors;
mod cert_spki;
mod cert_spki_refusal;
mod chain_authority;
mod chain_length;
mod chain_names;
mod chain_names_forged;
mod wallet_names;
mod chain_validity;
mod crypto_requests;
mod der_build;
mod fake_server;
mod flight_alert;
mod flight_complete;
mod flight_order;
mod flight_verdict;
mod handshake_step;
mod hello_retry;
mod hello_retry_records;
mod hkdf_label;
mod hkdf_label_shape;
mod inner_plain;
mod inner_plain_edges;
mod key_share_offer;
mod live_relay;
mod p256_share;
mod rfc8448_flight;
mod schedule_records;
mod schedule_rfc8448;
mod schedule_syscall;
mod schedule_syscall_keys;
mod server_hello;
mod server_hello_partial;
mod server_hello_refusal;
mod server_hello_vectors;
mod sha384_long_body;
mod step_plain;
mod stream_records;
mod tls13_only;
