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

//! shutdown on a stream outside the family. net.sockets has no half-close
//! at all; net.anon shuts both sides or neither.

use crate::linux::abi::errno;

use super::super::policy::refuse;
use super::super::sock::{Backend, Sock, Via};

/// The answer for socket `s`, when its stream is outside the family.
pub(super) fn outside(s: &mut Sock, rd: bool, wr: bool) -> Option<u64> {
    if let Some(Via::Sockets(_)) = s.svc.as_ref().map(Backend::via) {
        return Some(refuse(
            "shutdown of a stream outside the family: net.sockets has no half-close",
            errno::EOPNOTSUPP,
        ));
    }
    if let Some(Via::Anon) = s.svc.as_ref().map(Backend::via) {
        return Some(match s.shut_anon(rd, wr) {
            Ok(()) => errno::ok(0),
            Err(e) => refuse(
                "shutdown of one side of a stream outside the family: net.anon has no half-close",
                e,
            ),
        });
    }
    None
}
