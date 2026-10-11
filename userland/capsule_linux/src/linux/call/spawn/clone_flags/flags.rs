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

//! The clone flags, by Linux's numbers. Pure, so the host proofs hold them.

pub(super) const CLONE_VM: u64 = 0x100;
pub(super) const CLONE_FS: u64 = 0x200;
pub(super) const CLONE_FILES: u64 = 0x400;
pub(super) const CLONE_SIGHAND: u64 = 0x800;
pub(super) const CLONE_PIDFD: u64 = 0x1000;
pub(super) const CLONE_VFORK: u64 = 0x4000;
pub(super) const CLONE_THREAD: u64 = 0x1_0000;
pub(super) const CLONE_NEWNS: u64 = 0x2_0000;
pub(super) const CLONE_DETACHED: u64 = 0x40_0000;
const CLONE_NEWCGROUP: u64 = 0x200_0000;
const CLONE_NEWUTS: u64 = 0x400_0000;
const CLONE_NEWIPC: u64 = 0x800_0000;
pub(super) const CLONE_NEWUSER: u64 = 0x1000_0000;
pub(super) const CLONE_NEWPID: u64 = 0x2000_0000;
const CLONE_NEWNET: u64 = 0x4000_0000;

/// Every flag that asks for a namespace of its own.
pub(super) const NEW_NAMESPACE: u64 = CLONE_NEWNS
    | CLONE_NEWCGROUP
    | CLONE_NEWUTS
    | CLONE_NEWIPC
    | CLONE_NEWUSER
    | CLONE_NEWPID
    | CLONE_NEWNET;

/// The flags the legacy clone call reads: its lower 32 bits.
pub fn flags_of(raw: u64) -> u64 {
    raw & 0xffff_ffff
}
