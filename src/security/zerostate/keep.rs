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

//! The kernel's half of "nothing persists unless the person asked".
//!
//! The file service already refuses to keep a file on a boot that keeps
//! nothing, but any holder of StoreWrite can call the store write directly.
//! This check sits in that call, so the promise no longer rests on one
//! capsule behaving. Keeping starts off at every boot and is turned on only
//! by the policy capsule, the moment `Persistent` becomes true.

use alloc::vec;
use core::sync::atomic::{AtomicBool, Ordering};

use super::keep_rule::{after, may_arm, may_write, Policy};

static ARMED: AtomicBool = AtomicBool::new(false);

/// Whether this boot keeps: what vfs asks before it writes anything down.
pub(crate) fn keeping() -> bool {
    ARMED.load(Ordering::Acquire)
}

/// The policy service's ask to turn keeping on (`on`) or leave it. False,
/// said on serial, when the caller is not the policy capsule the kernel
/// spawned, or the ask is to turn it off.
pub(crate) fn arm(on: bool) -> bool {
    let state = crate::userspace::capsule_policy::shared_state();
    let policy = Policy { pid: state.pid(), alive: state.is_alive() };
    let caller = crate::process::current_pid().unwrap_or(0);
    let granted = may_arm(caller, policy, on);
    let was = ARMED.load(Ordering::Acquire);
    ARMED.store(after(was, granted), Ordering::Release);
    crate::sys::serial::print(match (granted, was) {
        (true, false) => b"[ZS] keeping turned on: the person chose it, the policy service says\n",
        (true, true) => b"[ZS] keeping already on\n",
        (false, _) if on => b"[ZS] keep refused: only the policy service turns keeping on\n",
        (false, _) => b"[ZS] keep refused: keeping is not turned off within a boot\n",
    });
    granted
}

/// True when `new` may be written at store sector `lba`. Before keeping is
/// on, the sectors are read first, from the disk the write will land on, and
/// only an erase passes; a read that fails refuses. Each answer that is not
/// a plain kept write is said on serial, so a boot test can find it.
pub(crate) fn store_write_allowed(lba: u64, new: &[u8]) -> bool {
    let armed = ARMED.load(Ordering::Acquire);
    if armed {
        return true;
    }
    let mut old = vec![0u8; new.len()];
    let read = crate::hardware::block_device::read_store(lba, &mut old).is_ok();
    let ok = read && may_write(armed, &old, new);
    crate::sys::serial::print(match (read, ok) {
        (_, true) => b"[ZS] store erase on a boot that keeps nothing\n",
        (false, _) => b"[ZS] store write refused: the sectors could not be read\n",
        (true, false) => b"[ZS] store write refused: this boot keeps nothing\n",
    });
    ok
}
