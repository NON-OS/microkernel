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

//! Which capability admits which syscall.

mod admin;
mod crypto;
mod graphics;
mod mk;
mod mk_device;
mod mk_ipc;
mod mk_platform;
mod mk_proc;
mod mk_surface;
mod mk_trust;

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

/// Total cap-table over `SyscallNumber`. `launcher` says whether the caller
/// is the desktop shell the kernel spawned, or a capsule SERVES pairs with a
/// listing; only MkAppLaunch reads it. Every caller passes a fixed answer:
/// the allowlist asks once for each, and the gate asks the live question
/// only when the two differ (`allowlist::decide`).
pub(super) fn is_allowed(
    caps: &CapabilityToken,
    number: SyscallNumber,
    launcher: bool,
) -> bool {
    crypto::check(caps, number)
        .or_else(|| admin::check(caps, number))
        .or_else(|| mk::check(caps, number))
        .or_else(|| mk_proc::check(caps, number))
        .or_else(|| mk_ipc::check(caps, number))
        .or_else(|| mk_device::check(caps, number))
        .or_else(|| mk_platform::check(caps, number))
        .or_else(|| mk_trust::check(caps, number, launcher))
        .or_else(|| mk_surface::check(caps, number))
        .or_else(|| graphics::check(caps, number))
        .unwrap_or(false)
}
