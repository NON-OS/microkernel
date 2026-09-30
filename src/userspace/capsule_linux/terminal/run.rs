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
//! `MkToolRun("tool.qwen", tier)`: the personality, as the caller's child,
//! in the first free terminal-run slot, asked to run that tier on the
//! caller's terminal. Neither the tier word nor the request is logged.

use super::super::install::spawn_role;
use super::roles::TERMINAL;
use super::{slots, tier};
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
use crate::process::signal::SIGKILL;
use crate::process::ProcessState;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

/// Start tier `word` for the calling process. Returns the child's pid, or
/// EINVAL for a word outside the allowlist (before anything is spawned),
/// EBUSY when every slot is held, EPERM without a calling process, ENOENT
/// when this image carries no personality, and EINVAL for a spawn the
/// verified path refused.
pub fn run_tier_for_caller(word: &[u8]) -> Result<u32, i64> {
    let Some(index) = tier::parse(word) else {
        crate::sys::serial::print(b"[LINUX-TERM] refused: unknown tier\n");
        return Err(ERRNO_INVAL);
    };
    let argv = tier::argv(index).ok_or(ERRNO_INVAL)?;
    let parent = crate::process::current_pid().ok_or(ERRNO_PERM)?;
    let Some(slot) = slots::reserve(parent, argv) else {
        crate::sys::serial::print(b"[LINUX-TERM] refused: every slot is busy\n");
        return Err(ERRNO_BUSY);
    };
    match spawn_role(&TERMINAL[slot]) {
        Ok(pid) => Ok(outlived(parent, pid)),
        Err(e) => {
            slots::cancel(slot);
            crate::sys::serial::print(b"[LINUX-TERM] spawn refused\n");
            Err(match e {
                SpawnError::FeatureDisabled => ERRNO_NOENT,
                SpawnError::EndpointCollision => ERRNO_BUSY,
                _ => ERRNO_INVAL,
            })
        }
    }
}

/*
 * The caller ended (killed from another CPU) while this spawn was under way.
 * Its teardown is a zombie before it ends its runs, so either it saw this
 * run held and ended it, or it is a zombie here: end the run, which no one
 * would read, and hand back the pid for a caller that is not there.
 */
fn outlived(parent: u32, pid: u32) -> u32 {
    let gone = crate::process::get_process(parent).map_or(true, |pcb| {
        matches!(*pcb.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
    });
    if gone {
        crate::process::exit::teardown(pid, 128 + SIGKILL as i32, true);
    }
    pid
}
