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
mod cert_message;
mod cert_message_refusal;
mod cert_message_vectors;
mod cert_spki;
mod cert_spki_refusal;
mod chain_authority;
mod chain_names;
mod chain_validity;
mod handshake_step;
mod hello_retry;
mod hello_retry_records;
mod hkdf_label;
mod hkdf_label_shape;
mod inner_plain;
mod inner_plain_edges;
mod server_hello;
mod server_hello_partial;
mod server_hello_refusal;
mod server_hello_vectors;
mod step_plain;
