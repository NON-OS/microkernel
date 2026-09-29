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
 * No internet sockets for a family that holds a model.
 *
 * The personality holds no Network capability, so the kernel already
 * refuses its every call to net.sockets. These refuse sooner and say why:
 * a new internet socket is EACCES once a model is held, and a model is
 * not opened while an internet socket is. Unix sockets stay: they reach
 * only the family and the display.
 */

use crate::linux::abi::errno;
use crate::linux::file::models::held;

use super::policy::refuse;
use super::sock::{self, Domain};
use super::sockaddr::AF_INET;

/* Some(errno) when an internet socket may not be made now. */
pub fn refuse_inet() -> Option<u64> {
    held().then(|| refuse("net.sockets: a family holding a model has no network", errno::EACCES))
}

/* A socket() of `family` refused now, or None to go on and make it. */
pub fn refuse_socket(family: u64) -> Option<u64> {
    (family == u64::from(AF_INET)).then(refuse_inet).flatten()
}

/* Whether any internet socket is open anywhere in the family. */
pub fn any_inet() -> bool {
    sock::with(|t| t.iter().any(|(_, s)| s.domain == Domain::Inet))
}
