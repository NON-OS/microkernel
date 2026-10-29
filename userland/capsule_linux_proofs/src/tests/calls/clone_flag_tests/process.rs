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

//! A clone that makes a process: vfork's clone, with or without a pidfd, is
//! made as vfork makes it; a child sharing the memory of a parent that runs
//! on is not; and the legacy call's one word is a tid or a pidfd, not both.

use super::super::super::random::Regs;
use super::made::{DETACHED, FILES, PIDFD, SIGCHLD, THREAD, VFORK, VM};
use crate::linux::abi::errno::EINVAL;
use crate::linux::call::clone_flags::flags_of;
use crate::linux::call::vfork_flags::{CLONE_PARENT_SETTID, SERVED};
use crate::linux::call::vfork_plan::{plan, Plan, Refused};

#[test]
fn qprocess_clone_is_vfork_with_a_pidfd() {
    // Telegram Desktop's QProcess on the owner's HP: "unserved clone: flags
    // beyond a copied process, flags 0x5111".
    assert_eq!(VM | PIDFD | VFORK | SIGCHLD, 0x5111);
    assert_eq!(plan(0x5111), Ok(Plan { signal: 17, parks: true, pidfd: true }));
    // Go's os/exec and musl's posix_spawn, made as before.
    assert_eq!(plan(VM | VFORK | SIGCHLD), Ok(Plan { signal: 17, parks: true, pidfd: false }));
    // forkfd's own fork, a pidfd without vfork.
    assert_eq!(plan(PIDFD | SIGCHLD), Ok(Plan { signal: 17, parks: false, pidfd: true }));
}

#[test]
fn a_child_sharing_a_running_parents_memory_stays_refused() {
    let why = Refused::Unserved("clone: a process sharing its parent's memory");
    for flags in [VM, VM | SIGCHLD, VM | PIDFD | SIGCHLD] {
        assert_eq!(plan(flags), Err(why), "{flags:#x}");
    }
    let beyond = Refused::Unserved("clone: flags beyond a copied process");
    assert_eq!(plan(FILES | PIDFD | SIGCHLD), Err(beyond));
}

#[test]
fn the_legacy_word_is_a_tid_or_a_pidfd_never_both() {
    let both = VM | PIDFD | VFORK | CLONE_PARENT_SETTID | SIGCHLD;
    assert_eq!(plan(both), Err(Refused::Linux(EINVAL)));
    assert_eq!(plan(PIDFD | DETACHED | SIGCHLD), Err(Refused::Linux(EINVAL)));
    assert_eq!(plan(VM | VFORK | 65), Err(Refused::Linux(EINVAL)));
}

#[test]
fn random_flags_make_only_what_a_copy_can_be() {
    let mut r = Regs::new(0x7066_6466_5f63_6c6f);
    for _ in 0..200_000 {
        let flags = flags_of(r.any() & r.any() & r.any()) & !THREAD;
        let Ok(made) = plan(flags) else { continue };
        assert_eq!(flags & !SERVED, 0, "{flags:#x}");
        assert!(flags & VM == 0 || flags & VFORK != 0, "{flags:#x}");
        assert!(flags & PIDFD == 0 || flags & CLONE_PARENT_SETTID == 0, "{flags:#x}");
        assert_eq!((made.parks, made.pidfd), (flags & VFORK != 0, flags & PIDFD != 0));
        assert_eq!(u64::from(made.signal), flags & 0xff);
    }
}
