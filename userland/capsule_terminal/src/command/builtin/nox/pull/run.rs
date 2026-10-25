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

use super::super::ensure_pid::ensure_pid;
use super::ctx::Ctx;
use super::walk::walk;
use super::{args, progress, store};
use crate::command::builtin::direct_gate::{refusal_line, PULL};
use crate::command::builtin::offline_probe::offline;
use crate::term::cwd::resolve;
use crate::term::state::State;

#[path = "run_file.rs"]
mod run_file;
#[path = "run_store.rs"]
mod run_store;
#[path = "run_verify.rs"]
mod run_verify;

pub(super) use run_file::one_file;

pub fn run(state: &mut State, argv: &[&[u8]]) -> bool {
    let a = match args::parse(argv) {
        Ok(a) => a,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let refused = nonos_route_link::direct_refusal();
    if let Some(line) = refusal_line(b"pull", PULL, refused) {
        state.scrollback.push_error(&line);
        return false;
    }
    if let Some(line) = offline(b"pull") {
        state.scrollback.push_error(&line);
        return false;
    }
    let ip = match super::resolve::resolve_host(&a.target.hostname) {
        Ok(ip) => ip,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let pid = ensure_pid(state);
    let dest = resolve(state.cwd.as_bytes(), &a.dest);
    let mut tally = progress::Tally { ok: 0, skipped: 0, failed: 0 };
    if a.target.is_dir {
        store::mkdir(pid, &dest);
        let mut count = 0u32;
        walk(state, &Ctx { pid, ip, args: &a }, &a.target.path, &dest, 0, &mut count, &mut tally);
    } else {
        let mut conn = None;
        one_file(state, &Ctx { pid, ip, args: &a }, &mut conn, &a.target.path, &dest, &mut tally);
        if let Some(c) = conn {
            c.close();
        }
    }
    state.scrollback.push_line(&progress::summary(&tally));
    true
}
