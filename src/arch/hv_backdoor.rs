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

//! The VMware backdoor for the syscall layer: one call from the allowed set,
//! on a host that answers it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HvBackdoorError {
    /// No backdoor here: another hypervisor, a physical machine, or an
    /// architecture without port I/O.
    NoHost,
    /// A command, or an argument, outside the allowed set.
    #[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
    Refused,
}

/// EAX, EBX, ECX and EDX as the host left them.
#[cfg(target_arch = "x86_64")]
pub(crate) fn hv_backdoor(cmd: u32, arg: u32) -> Result<[u32; 4], HvBackdoorError> {
    use crate::arch::x86_64::hypervisor::{backdoor, BackdoorError};
    backdoor(cmd, arg).map_err(|e| match e {
        BackdoorError::NoHost => HvBackdoorError::NoHost,
        BackdoorError::Refused => HvBackdoorError::Refused,
    })
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn hv_backdoor(_cmd: u32, _arg: u32) -> Result<[u32; 4], HvBackdoorError> {
    Err(HvBackdoorError::NoHost)
}

/// Whether a backdoor answers here, so its drivers are worth starting.
#[cfg(target_arch = "x86_64")]
pub fn hv_backdoor_present() -> bool {
    crate::arch::x86_64::hypervisor::backdoor_present()
}

#[cfg(not(target_arch = "x86_64"))]
pub fn hv_backdoor_present() -> bool {
    false
}
