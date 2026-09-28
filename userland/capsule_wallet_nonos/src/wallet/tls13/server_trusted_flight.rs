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

/*
 * Everything that has to hold before a byte of application data goes to
 * the server: a chain that reaches the pinned root with every signature
 * good, a leaf naming the host and valid now, the leaf's key signing this
 * handshake, and the server's Finished. The data path asks this, not the
 * probe, so a status line can never stand in for a check.
 */

use super::flight::ClientFlight;

pub fn server_trusted_flight(client: &ClientFlight, bytes: &[u8], host: &[u8], now: u64) -> bool {
    super::server_chain_flight::server_chain_flight(client, bytes)
        && super::server_anchor_flight::server_anchor_flight(client, bytes)
        && super::server_signature_flight::server_signature_flight(client, bytes)
        && super::server_hostname_flight::server_hostname_flight(client, bytes, host)
        && super::server_validity_flight::server_validity_flight(client, bytes, now)
        && super::server_cert_verify_flight::server_cert_verify_flight(client, bytes)
        && super::server_finished_flight::server_finished_flight(client, bytes)
}
