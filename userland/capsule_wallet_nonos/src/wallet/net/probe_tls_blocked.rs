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

//! A probe that got nowhere: every flag down.

use super::probe_tls_rpc::TlsProbe;

impl TlsProbe {
    pub fn blocked() -> Self {
        Self {
            server_hello: false,
            encrypted_record: false,
            certificate: false,
            chain: false,
            anchor: false,
            signature: false,
            hostname: false,
            validity: false,
            finished: false,
            client_finished: false,
            chain_id: false,
        }
    }
}
