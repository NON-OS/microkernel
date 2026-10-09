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

//! What a refused handshake amounts to, for the person reading the page.

use crate::browser::tls13::CertProblem;

/*
 * Recorded beside the fetch's error, never in place of it: the error string
 * is what the retry rule and the traces key on, and it stays
 * "tls handshake refused". This is only what the page says.
 */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TlsWhy {
    /// The certificate was refused. The problem is `None` when no reason a
    /// person can act on was found (a signature that did not verify); the
    /// clock is the `YYYYMMDDhhmmss` the check ran at, zero if unreadable.
    Cert(Option<CertProblem>, u64),
    /// The server answered a TLS 1.3 hello as a TLS 1.2 server would.
    Tls12Only,
}
