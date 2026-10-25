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

//! Requests: a 24-byte header then the body, in one page of guest memory.

pub const HEADER_VERSION: u32 = 0x0001_0001;
pub const HEADER_BYTES: usize = 24;
/// The largest body sent or read back (the host version's).
pub const BODY_MAX: usize = 16;
/// What a request's status reads before the host answers it: a request the
/// host never saw is a failure, not a success.
pub const RC_UNANSWERED: i32 = -1;

pub const GET_MOUSE_STATUS: u32 = 1;
pub const SET_MOUSE_STATUS: u32 = 2;
pub const GET_HOST_VERSION: u32 = 4;
pub const ACKNOWLEDGE_EVENTS: u32 = 41;
pub const CTL_GUEST_FILTER_MASK: u32 = 42;
pub const REPORT_GUEST_INFO: u32 = 50;
pub const SET_GUEST_CAPABILITIES: u32 = 56;

/// The interface version the guest info names, VMMDEV_VERSION.
pub const INTERFACE_VERSION: u32 = 0x0001_0004;
/// An operating system the host has no name for, 64-bit.
pub const OS_TYPE_UNKNOWN_X64: u32 = 0x100;

/// VMMDevMemory: its size and version words, and the pending flag.
pub const MEMORY_SIZE_MIN: u32 = 32;
pub const MEMORY_VERSION: u32 = 1;
pub const MEMORY_HAVE_EVENTS: usize = 8;
