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

//! `execve`: the same process, a different program.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::exec_args::vector;
use super::exec_clear::clear;
use super::exec_load::load_over;
use super::exec_resolve::resolve;

pub fn execve(guest: &mut Guest, pid: u32, path: u64, argv: u64, envp: u64) -> Answer {
    let name = match crate::linux::file::name_of(guest, path) {
        Ok(name) => name,
        Err(e) => return Answer::value(errno::fail(e)),
    };
    /*
     * argv and envp live in the memory that is about to be unmapped, so they
     * are copied out here and not one step later.
     */
    let both = vector(&*guest, argv).and_then(|args| Ok((args, vector(&*guest, envp)?)));
    let (args, env) = match both {
        Ok(both) => both,
        Err(e) => return Answer::value(errno::fail(e)),
    };
    /*
     * Found, followed through any `#!` line, and proved at every step, all
     * while the caller still has an address space to be told no in.
     */
    let program = match resolve(&guest.links, &guest.cwd, &name, &args) {
        Ok(p) => p,
        Err(e) => return Answer::value(e),
    };
    super::exec_threads::reap(guest, pid);
    clear(guest);
    match load_over(guest, pid, &program, &env) {
        Some(()) => {
            released(guest, pid);
            Answer::Park
        }
        None => Answer::value(errno::fail(errno::ENOEXEC)),
    }
}

/// The new program keeps what Linux keeps of the old one's signals, and a
/// vfork parent waiting on this exec is let go with the child's pid.
fn released(guest: &mut Guest, pid: u32) {
    guest.signals.exec_reset(pid);
    if let Some(parent) = guest.signals.vfork.take() {
        let child = u64::from(crate::linux::serve::guest_pid(guest.pid));
        let _ = nonos_libc::mk_foreign_reply(parent, child);
    }
}
