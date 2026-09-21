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

//! Reaching the peer's leaf certificate out of the Certificate message.

use super::types::ServerComplete;

impl ServerComplete {
    /// The peer's own certificate, DER encoded. `None` if the peer sent no
    /// Certificate message or its list is malformed.
    pub fn leaf(&self) -> Option<&[u8]> {
        crate::cert_at::cert_at(&self.certificates, 0)
    }
}
