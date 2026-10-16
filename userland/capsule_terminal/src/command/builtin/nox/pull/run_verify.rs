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

//! `--verify`: a pulled body held to the checksum the server keeps beside it.

use super::super::conn::Conn;
use super::super::ctx::Ctx;
use super::super::{progress, verify};
use crate::term::state::State;

/// Whether `body` matches; a mismatch is said and counted as a failure.
pub(super) fn verified(
    state: &mut State,
    ctx: &Ctx<'_>,
    conn: &mut Option<Conn>,
    path: &[u8],
    extra: &[u8],
    body: &[u8],
    tally: &mut progress::Tally,
) -> bool {
    let (ip, a) = (ctx.ip, ctx.args);
    if let Err(e) = verify::check(conn, ip, a.target.port, &a.target.host, path, extra, body) {
        state.scrollback.push_error(e.as_bytes());
        tally.failed += 1;
        return false;
    }
    true
}
