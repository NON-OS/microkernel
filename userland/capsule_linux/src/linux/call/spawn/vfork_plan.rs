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

//! How a clone without CLONE_THREAD makes its process, or why it makes none.
//! Pure, so the host proofs hold each answer.

use super::clone_flags::clone_valid;
use super::vfork_flags::SERVED;
use super::vfork_flags::{CLONE_PARENT_SETTID, CLONE_PIDFD, CLONE_VFORK, CLONE_VM, CSIGNAL};
use crate::linux::abi::errno;
use crate::linux::guest::sigstate::NSIG;

/// A process clone that is made.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Plan {
    /// The signal the child raises at its parent when it ends.
    pub signal: u8,
    /// CLONE_VFORK: the parent parks until the child execs or ends.
    pub parks: bool,
    /// CLONE_PIDFD: the parent is given a pidfd for the child.
    pub pidfd: bool,
}

/// A process clone that is not made: Linux's own errno, or ENOSYS with what
/// this personality does not make, which the log names.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refused {
    Linux(i64),
    Unserved(&'static str),
}

/// CLONE_VM is made only with CLONE_VFORK, vfork's own clone (Go's os/exec,
/// musl's posix_spawn, Qt's QProcess): the child is a copy, but it may only
/// exec or exit while its parent sleeps, so neither can tell.
pub fn plan(flags: u64) -> Result<Plan, Refused> {
    let has = |f: u64| flags & f != 0;
    /* The legacy call writes the pidfd at parent_tid; Linux refuses both first. */
    if has(CLONE_PIDFD) && has(CLONE_PARENT_SETTID) {
        return Err(Refused::Linux(errno::EINVAL));
    }
    clone_valid(flags).map_err(Refused::Linux)?;
    if flags & !SERVED != 0 {
        return Err(Refused::Unserved("clone: flags beyond a copied process"));
    }
    if has(CLONE_VM) && !has(CLONE_VFORK) {
        return Err(Refused::Unserved("clone: a process sharing its parent's memory"));
    }
    let signal = flags & CSIGNAL;
    if signal > NSIG as u64 {
        return Err(Refused::Linux(errno::EINVAL));
    }
    Ok(Plan { signal: signal as u8, parks: has(CLONE_VFORK), pidfd: has(CLONE_PIDFD) })
}
