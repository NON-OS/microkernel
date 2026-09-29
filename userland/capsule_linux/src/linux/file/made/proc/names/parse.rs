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

/* What each path under /proc names. */

use super::super::super::synth::num;
use super::super::super::view::{Proc, View};
use super::at::At;
use super::lists::{FILES, SYSTEM};
use super::node::{number, open_fd};

pub fn parse<'a>(v: &'a View, rest: &'a [&'a [u8]]) -> Option<At<'a>> {
    let Some((first, more)) = rest.split_first() else {
        return Some(At::Root);
    };
    match (*first, more) {
        (b"self", []) => Some(At::Link(num(u64::from(v.me)))),
        (b"thread-self", []) => Some(At::Link(alloc::format!("{}/task/{}", v.me, v.thread).into())),
        (b"mounts", []) => Some(At::Link(b"self/mounts".to_vec())),
        (b"sys", _) => Some(At::Sysctl(more)),
        (name, []) if SYSTEM.contains(&name) => Some(At::System(name)),
        (pid, _) => {
            let proc = v.find(number(pid)?)?;
            in_pid(proc, proc.ns, more, true)
        }
    }
}

fn in_pid<'a>(proc: &'a Proc, tid: u32, more: &'a [&'a [u8]], leader: bool) -> Option<At<'a>> {
    match more {
        [] => Some(At::PidDir((!leader).then_some(tid))),
        [b"task"] if leader => Some(At::Task(proc)),
        [b"task", t, rest @ ..] if leader => {
            let t = number(t)?;
            proc.tids.iter().any(|(ns, _)| *ns == t).then_some(())?;
            in_pid(proc, t, rest, false)
        }
        [b"fd"] => Some(At::FdDir(proc)),
        [b"fd", n] => Some(At::Fd { proc, fd: open_fd(proc, n)? }),
        [b"fdinfo"] => Some(At::FdInfoDir(proc)),
        [b"fdinfo", n] => Some(At::FdInfo { proc, fd: open_fd(proc, n)? }),
        [b"root"] => Some(At::Link(b"/".to_vec())),
        [b"cwd"] => Some(At::Link(proc.cwd.clone())),
        [b"exe"] => (!proc.exe.path.is_empty()).then(|| At::Link(proc.exe.path.clone())),
        [file] if FILES.contains(file) => Some(At::Pid { proc, tid, file }),
        _ => None,
    }
}
