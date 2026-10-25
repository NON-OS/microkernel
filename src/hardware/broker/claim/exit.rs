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

//! A capsule's claims at its exit (L75). The exit does not wait for paths
//! still inside a claim: it can be reached from a fault, a signal or a spawn
//! that failed, where waiting is not safe. It moves each claim to Exiting, so
//! nothing more is granted or entered against it, and stops its device while
//! the entry still names it; the grants are drained after; the entry goes
//! with the last of the exit, any release and any path inside it.

extern crate alloc;

use alloc::vec::Vec;

use super::stage_rule::exited;
use super::state::claims;

/// The exit's first step: the claims it tears down, as (device, epoch), each
/// stopped from mastering the bus. A claim still being made is orphaned for
/// its call to take back (L66), and is not the exit's.
pub fn exit_begin(pid: u32) -> Vec<(u64, u64)> {
    let mut held = Vec::new();
    for c in claims().iter_mut().filter(|c| c.pid == pid) {
        let (stage, counted) = exited(c.stage);
        c.stage = stage;
        if counted {
            c.leavers = c.leavers.saturating_add(1);
            held.push((c.device_id, c.epoch));
        }
    }
    for &(device_id, _) in &held {
        super::quiesce::stop_bus_master(device_id);
    }
    held
}

/// The exit's last step, once its grants are drained: each claim it began on
/// loses the exit from its releases, and goes if nothing else is left in it;
/// the capsule's domain goes with the exit.
pub fn exit_end(pid: u32, held: &[(u64, u64)]) {
    for &(device_id, epoch) in held {
        let _ = super::release::settle((pid, device_id, epoch));
    }
    crate::hardware::broker::confine::detach_all(pid);
    super::wipe::step_all(true);
}
