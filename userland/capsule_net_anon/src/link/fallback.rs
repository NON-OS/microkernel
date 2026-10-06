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


//! When a link falls back from TLS 1.3 to TLS 1.2.
//!
//! A relay that answers a TLS 1.3 hello with protocol_version (70) or
//! handshake_failure (40) is one whose TLS stops at 1.2: it found no
//! version, or no suite, that it shares with a hello that offers only 1.3.
//! It gets one TLS 1.2 handshake on a fresh connection. Nothing else falls
//! back: not a timeout, not a bad certificate, not any other alert. And the
//! TLS 1.2 client refuses a server whose random says it could have done 1.3,
//! so a forged alert cannot downgrade a relay that can.

pub fn refuses_tls13(description: u8) -> bool {
    matches!(description, 40 | 70)
}
