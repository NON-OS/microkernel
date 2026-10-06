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

// Terminate a capsule by pid or by name: `kill <pid|name> [signal]` (signal
// defaults to 9). A name is looked up in the service registry, as typed and
// then as app., tool. and net., so `kill browser` ends app.browser. Routed
// through the capability-checked MkKill syscall, so it only reaches a pid the
// terminal is allowed to signal. `caps`/`ps` lists the pids.

use alloc::vec::Vec;
use nonos_libc::{mk_kill, mk_service_lookup};

use super::kill_target::{parse_u64, refused, resolve, target, Target, NAME_MAX};
use crate::term::state::State;
use crate::term::util::format_u64;

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(&arg) = args.first() else {
        state.scrollback.push_error(b"usage: kill <pid|name> [signal]");
        return false;
    };
    let mut buf = [0u8; NAME_MAX];
    let (pid, name) = match target(arg) {
        Target::Pid(pid) => (pid, None),
        Target::Name => match resolve(arg, &mut buf, lookup) {
            Some((len, pid)) => (pid, Some(&buf[..len])),
            None => {
                let mut line: Vec<u8> = Vec::new();
                line.extend_from_slice(b"kill: nothing running is called ");
                line.extend_from_slice(arg);
                state.scrollback.push_error(&line);
                return false;
            }
        },
        Target::Unreadable => {
            state.scrollback.push_error(b"kill: give a pid or a capsule name");
            return false;
        }
    };
    let sig = args.get(1).and_then(|a| parse_u64(a)).unwrap_or(9);
    let rc = mk_kill(pid, sig);
    if rc < 0 {
        state.scrollback.push_error(refused(rc));
        return false;
    }
    let mut line: Vec<u8> = Vec::new();
    line.extend_from_slice(b"killed ");
    if let Some(name) = name {
        line.extend_from_slice(name);
        line.push(b' ');
    }
    line.extend_from_slice(b"pid ");
    append_u64(&mut line, pid);
    state.scrollback.push_line(&line);
    true
}

// The registry's pid for a live service, if one answers to `name`.
fn lookup(name: &[u8]) -> Option<u64> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc == 0 && port != 0 && pid != 0).then_some(pid as u64)
}

fn append_u64(out: &mut Vec<u8>, v: u64) {
    let mut buf = [0u8; 24];
    let k = format_u64(v, &mut buf);
    out.extend_from_slice(&buf[..k]);
}
