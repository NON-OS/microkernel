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

//! What a guest is told when net.anon opens a stream, or does not.

use super::super::anon_ops::{
    E_BAD_LEN, E_DIRECTORY_STALE, E_NO_CIRCUIT, E_NO_DIRECTORY, E_NO_LINK, E_NO_PATH, E_NO_TCP,
    E_OK, E_TABLE_FULL,
};
use super::open::Reply;
use crate::linux::abi::errno::{ECONNREFUSED, EIO, ENETUNREACH, ENOBUFS};

/// The stream id an open was given, or the errno a connect answers.
pub fn opened(reply: Reply) -> Result<u16, i64> {
    match reply {
        Some((E_OK, &[lo, hi])) => Ok(u16::from_le_bytes([lo, hi])),
        Some((E_OK, _)) => Err(EIO),
        /*
         * net.anon has no directory, no relay to build through, no link to its
         * guard, or no circuit: there is no route out, which a tool has to
         * read as unreachable, not as a peer that refused.
         */
        Some((
            E_NO_TCP | E_NO_DIRECTORY | E_DIRECTORY_STALE | E_NO_PATH | E_NO_LINK | E_NO_CIRCUIT,
            _,
        )) => Err(ENETUNREACH),
        /*
         * This capsule, one caller for every guest, holds as many streams as
         * net.anon gives one caller (half its table). ENOBUFS fails the
         * connect at once and says why; EAGAIN would park a blocking connect
         * with nothing to wake it, and a non-blocking one reads it as a
         * connect in progress.
         */
        Some((E_TABLE_FULL, _)) => Err(ENOBUFS),
        Some((E_BAD_LEN, _)) => Err(ECONNREFUSED),
        Some(_) | None => Err(EIO),
    }
}
