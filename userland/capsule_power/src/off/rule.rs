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

//! Who may power the machine off through this capsule, pure. The desktop
//! shell alone: `power.off` is its request for the wiped power-off, and the
//! `power` port's reboot and shutdown are the same act by another frame.
//! Anyone may ask for a health check.

/// The kernel's EPERM and EINVAL, as a refusal carries them.
pub const EPERM: i32 = -1;
pub const EINVAL: i32 = -22;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Prepare soft-off and ask the kernel for AdminShutdown.
    Off,
    /// Answer with this status and leave the machine running.
    Refuse(i32),
}

/// A `power.off` request: well formed and from the shell, or refused.
pub fn verdict(well_formed: bool, from_shell: bool) -> Verdict {
    if !well_formed {
        Verdict::Refuse(EINVAL)
    } else if !from_shell {
        Verdict::Refuse(EPERM)
    } else {
        Verdict::Off
    }
}

/// Whether a `power` op may run for this sender: the health check for
/// anyone, reboot (2) and shutdown (3) for the shell alone.
pub fn op_allowed(op: u16, from_shell: bool) -> bool {
    match op {
        2 | 3 => from_shell,
        _ => true,
    }
}
