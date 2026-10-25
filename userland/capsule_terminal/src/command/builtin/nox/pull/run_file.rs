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

//! One file pulled: skipped when asked and unchanged, else fetched, checked and stored.

use super::super::conn::Conn;
use super::super::ctx::Ctx;
use super::super::{fetch, progress, store};
use super::run_store::store_body;
use super::run_verify::verified;
use crate::term::state::State;

pub(in super::super) fn one_file(
    state: &mut State,
    ctx: &Ctx<'_>,
    conn: &mut Option<Conn>,
    path: &[u8],
    dest: &[u8],
    tally: &mut progress::Tally,
) -> bool {
    let (pid, ip, a) = (ctx.pid, ctx.ip, ctx.args);
    if a.no_clobber && store::exists(pid, dest) {
        tally.skipped += 1;
        return true;
    }
    let extra = super::super::auth::extra_headers(&a.auth, &a.headers);
    if a.skip_unchanged {
        if let Some(local) = store::size(pid, dest) {
            if fetch::head_reuse(conn, ip, a.target.port, &a.target.host, path, &extra)
                == Some(local as usize)
            {
                tally.skipped += 1;
                return true;
            }
        }
    }
    if !store::has_free_slot(pid) {
        state.scrollback.push_error(b"pull: vfs store full");
        tally.failed += 1;
        return false;
    }
    match fetch::get_reuse(conn, ip, a.target.port, &a.target.host, path, &extra) {
        Ok(body) => {
            if a.verify && !verified(state, ctx, conn, path, &extra, &body, tally) {
                return false;
            }
            store_body(state, pid, dest, &body, tally)
        }
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            tally.failed += 1;
            false
        }
    }
}
