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

//! Capability inheritance policy for new PCBs.
//!
//! Syscall authority is now decided through `pcb.capability_token`;
//! `pcb.caps_bits` is the derived bitmap cache. This module produces
//! the inherited bitmap that seeds both — the cache directly, and
//! the initial token via `process::caps::new_token`. The bits stay
//! in the `crate::capabilities::Capability` namespace.
//!
//! One policy knob lives here:
//!   - `AMBIENT_CAPS`: init's production-ambient set, also the
//!     upper bound on inheritance. Hardware authority (Driver,
//!     DeviceEnum, Mmio, Irq, Dma, Pio), Admin, Debug, and the
//!     graphics caps are never ambient. They must be granted by
//!     each capsule's spawn spec.
//! A compile-time assertion in ambient.rs proves `AMBIENT_CAPS` carries
//! none of the forbidden bits.

use core::sync::atomic::Ordering;

use super::super::types::Pid;
use super::ambient::AMBIENT_CAPS;
use super::types::PROCESS_TABLE;

pub(super) fn compute_inherited_caps(pid: Pid, parent_pid: Pid) -> u64 {
    if pid == 1 {
        return apply_inherit_bound(AMBIENT_CAPS);
    }
    match PROCESS_TABLE.find_by_pid(parent_pid) {
        Some(parent) => apply_inherit_bound(parent.caps_bits.load(Ordering::Acquire)),
        None => 0,
    }
}

// Sole producer of inherited caps. Both `compute_inherited_caps`
// and the fork/clone path go through here.
pub(crate) fn apply_inherit_bound(parent_caps: u64) -> u64 {
    parent_caps & inheritable_bound()
}

// Children inherit only the small ambient set. Hardware and admin
// authority do not ride inheritance; spawners that need them carry
// explicit `caps_bits` in their `CapsuleSpec`.
fn inheritable_bound() -> u64 {
    AMBIENT_CAPS
}
