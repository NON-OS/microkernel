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

//! The clone flags a new process may be made with, as Linux numbers them.

pub const CSIGNAL: u64 = 0xff;
pub const CLONE_VM: u64 = 0x100;
pub const CLONE_VFORK: u64 = 0x4000;
pub const CLONE_PARENT_SETTID: u64 = 0x10_0000;
pub const CLONE_CHILD_CLEARTID: u64 = 0x20_0000;
pub const CLONE_DETACHED: u64 = 0x40_0000;
pub const CLONE_UNTRACED: u64 = 0x80_0000;
pub const CLONE_CHILD_SETTID: u64 = 0x100_0000;
/// Every flag honoured for a new process; CLONE_DETACHED and CLONE_UNTRACED
/// are ones Linux itself ignores.
pub const SERVED: u64 = CSIGNAL
    | CLONE_VM
    | CLONE_VFORK
    | CLONE_PARENT_SETTID
    | CLONE_CHILD_CLEARTID
    | CLONE_DETACHED
    | CLONE_UNTRACED
    | CLONE_CHILD_SETTID;
