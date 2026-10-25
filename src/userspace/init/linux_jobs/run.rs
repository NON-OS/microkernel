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

//! Starting what a queued run asked for, said on the log.

use alloc::string::String;

use crate::sys::serial::{print, println};
use crate::userspace::capsule_linux::{spawn_run, spawn_run_net};

/// How a queued run starts.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum How {
    /// The run role, said with its package.
    Named,
    /// The run role, said without its package: a shipped tier a terminal
    /// asked for is named nowhere on the log.
    Quiet,
    /// app.linux.run.net, for a run that found its program's policy has
    /// NET_ANON and asked to start again (MkLinuxRunNet). The first start
    /// said the package, or kept it off, already.
    Net,
}

/// Queue the run of `package` again with the network.
pub(crate) fn request_net_run(package: String) -> bool {
    super::queue::push(super::queue::Job::Run(package, How::Net))
}

pub(super) fn run(package: &str, how: How) {
    let spawn = if how == How::Net { spawn_run_net } else { spawn_run };
    let said: &[u8] = match super::super::app_choice::linux_off() {
        true => b"[LINUX-RUN] Linux turned off at setup, not started: ",
        false if spawn(package).is_ok() => b"[LINUX-RUN] started ",
        false => b"[LINUX-RUN] refused ",
    };
    print(said);
    println(match how {
        How::Named => package.as_bytes(),
        How::Quiet => b"a Qwen window",
        How::Net => b"the same run, with the network",
    });
}
