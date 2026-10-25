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
//! `MkCapsOf(pid, bit, sent_ms)`: for a SpawnBroker holder (the installer),
//! whether the process `pid` names, the attested sender of a message it
//! received at `sent_ms` (`mk_ipc_recv_from_at`'s enqueue time), holds one
//! askable right. 1 or 0, or an errno (`rule.rs`).

use core::sync::atomic::Ordering;

use super::ipc::enqueued_uptime;
use crate::capabilities::Capability;
use crate::process::core::types::ProcessState;
use rule::{answer, Target};

pub(crate) mod rule;

pub fn sys_caps_of(pid: u64, bit: u64, sent_ms: u64) -> i64 {
    let caps = crate::syscall::caps::current_caps_or_default();
    let broker = caps.is_valid() && caps.grants(Capability::SpawnBroker);
    let now = (crate::sys::clock::since_boot_ms(), crate::time::timestamp_millis());
    let target = u32::try_from(pid).ok().and_then(|pid| {
        crate::process::with_process(pid, |pcb| Target {
            live: !matches!(
                *pcb.state.lock(),
                ProcessState::Zombie(_) | ProcessState::Terminated(_)
            ),
            started_ms: enqueued_uptime(now.0, now.1, pcb.start_time_ms.load(Ordering::Relaxed)),
            caps: pcb.caps_bits.load(Ordering::Acquire),
        })
    });
    answer(broker, bit, target, sent_ms)
}
