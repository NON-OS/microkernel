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
 * The mark an install's download carries to net.anon: a SOCKS5 method
 * from the private range (RFC 1928 reserves X'80' to X'FE'), listed after
 * no-authentication and never chosen in reply. net.anon reads it as "this
 * stream is an install's" and hands the stream the circuit it keeps warm
 * for installs (its manager/warm_rule.rs). net.socks5 reads past it,
 * though an install never goes over Nym (`install_route`). The method
 * reply is read as for any greeting, and nothing else in the conversation
 * changes: the mark names no package and no host.
 */

use crate::socks::GREETING;

/* The private method that marks an install's stream. */
pub const METHOD_INSTALL: u8 = 0x80;

/* Version 5, two methods: no authentication, then the install mark. */
pub const INSTALL_GREETING: [u8; 4] = [5, 2, 0, METHOD_INSTALL];

/* The greeting a stream opens with: marked only for an install's. */
pub fn greeting(install: bool) -> &'static [u8] {
    if install {
        &INSTALL_GREETING
    } else {
        &GREETING
    }
}
