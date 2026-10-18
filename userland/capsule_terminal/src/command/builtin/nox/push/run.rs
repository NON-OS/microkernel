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
use super::super::pull::auth::extra_headers;
use super::super::pull::resolve::resolve_host;
use super::{args, walk};
use crate::command::builtin::direct_gate::{refusal_line, PUSH};
use crate::command::builtin::offline_probe::offline;
use crate::term::cwd::resolve;
use crate::term::state::State;

#[path = "run_file.rs"]
mod run_file;

use run_file::push_file;

pub fn run(state: &mut State, argv: &[&[u8]]) -> bool {
    let a = match args::parse(argv) {
        Ok(a) => a,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    if let Some(line) = refusal_line(b"push", PUSH, nonos_route_link::direct_refusal()) {
        state.scrollback.push_error(&line);
        return false;
    }
    if let Some(line) = offline(b"push") {
        state.scrollback.push_error(&line);
        return false;
    }
    let ip = match resolve_host(&a.target.hostname) {
        Ok(ip) => ip,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let pid = ensure_pid(state);
    let path = resolve(state.cwd.as_bytes(), &a.src);
    let extra = extra_headers(&a.auth, &a.headers);
    if a.target.is_dir {
        return walk::walk(state, pid, ip, &path, &a.target, &extra);
    }
    push_file(state, pid, ip, path, &a, extra)
}
