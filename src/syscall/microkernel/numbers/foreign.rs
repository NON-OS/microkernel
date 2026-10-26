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

//! Guests the caller supervises: the Linux personality's calls.

use crate::syscall::abi::tag4;

/// A process with no capabilities, supervised by the caller, for code never verified.
pub const SYS_FOREIGN_SPAWN: u64 = tag4(b"MFSP");
/// Give such a process an entry point and make it runnable.
pub const SYS_FOREIGN_START: u64 = tag4(b"MFST");
/// Wait for a guest of the caller's to make a refused syscall; take its registers.
pub const SYS_FOREIGN_WAIT: u64 = tag4(b"MFWT");
/// Answer one parked guest with the value its `rax` receives.
pub const SYS_FOREIGN_REPLY: u64 = tag4(b"MFRP");
/// Copy a parked guest's registers out, in `struct sigcontext` order.
pub const SYS_FOREIGN_CONTEXT: u64 = tag4(b"MFCX");
/// Answer a parked guest with a context: a signal handler, or its return.
pub const SYS_FOREIGN_SIGNAL: u64 = tag4(b"MFSG");
/// Stop a guest thread that is running its own code at its next timer tick,
/// and hand it over parked, so a signal can be delivered to it.
pub const SYS_FOREIGN_INTERRUPT: u64 = tag4(b"MFIN");
/// Back a span of a guest's address space with fresh frames.
pub const SYS_PEER_MAP: u64 = tag4(b"MPMP");
/// Copy bytes between the caller and a guest it supervises.
pub const SYS_PEER_COPY: u64 = tag4(b"MPCP");
/// Set the protection of pages a guest already has.
pub const SYS_PEER_PROTECT: u64 = tag4(b"MPPT");
/// A thread inside a guest, sharing its address space.
pub const SYS_FOREIGN_THREAD: u64 = tag4(b"MFTH");
/// The thread pointer a guest thread wakes with.
pub const SYS_PEER_TLS: u64 = tag4(b"MPTL");
/// A second process holding a guest's register state.
pub const SYS_FOREIGN_FORK: u64 = tag4(b"MFFK");
/// Take pages back from a guest, which exec needs.
pub const SYS_PEER_UNMAP: u64 = tag4(b"MPUN");
/// Replace the program a parked guest is running.
pub const SYS_FOREIGN_EXEC: u64 = tag4(b"MFEX");
/// The Linux run role asks to be started again with Network.
pub const SYS_LINUX_RUN_NET: u64 = tag4(b"MLRN");
