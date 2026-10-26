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

//! IPC, memory, processes, time and a process's own output.

use crate::syscall::abi::tag4;

pub const SYS_IPC_SEND: u64 = tag4(b"MISD");
pub const SYS_IPC_RECV: u64 = tag4(b"MIRC");
pub const SYS_IPC_CALL: u64 = tag4(b"MICL");
pub const SYS_IPC_RECV_FROM: u64 = tag4(b"MIRF");
pub const SYS_IPC_REPLY: u64 = tag4(b"MIRY");
/// A receive that names the request, for MkIpcReplyTo to answer.
pub const SYS_IPC_RECV_REQ: u64 = tag4(b"MIRQ");
/// A reply to the request MkIpcRecvReq named, and only that one.
pub const SYS_IPC_REPLY_TO: u64 = tag4(b"MIRT");
pub const SYS_IPC_SEND_TO_PID: u64 = tag4(b"MISP");
pub const SYS_SERVICE_LOOKUP: u64 = tag4(b"MSVL");
pub const SYS_THREAD_GROUP: u64 = tag4(b"MTGR");
pub const SYS_SERVICE_REGISTER: u64 = tag4(b"MSVR");
pub const SYS_MMAP: u64 = tag4(b"MMAP");
pub const SYS_MUNMAP: u64 = tag4(b"MUMP");
pub const SYS_CAPSULE_LOAD: u64 = tag4(b"MCLD");
pub const SYS_CAPSULE_VERIFY: u64 = tag4(b"MCVF");
pub const SYS_EXIT: u64 = tag4(b"MEXT");
pub const SYS_PID_ALIVE: u64 = tag4(b"MPAL");
pub const SYS_WAIT: u64 = tag4(b"MWAT");
pub const SYS_KILL: u64 = tag4(b"MKIL");
pub const SYS_GETPID: u64 = tag4(b"MGPD");
pub const SYS_ARGS: u64 = tag4(b"MKAR");
pub const SYS_THREAD_SPAWN: u64 = tag4(b"MTSP");
pub const SYS_SET_TLS: u64 = tag4(b"MSTB");
pub const SYS_YIELD: u64 = tag4(b"MYLD");
pub const SYS_FUTEX_WAIT: u64 = tag4(b"MFTW");
pub const SYS_FUTEX_WAKE: u64 = tag4(b"MFTK");
pub const SYS_TIME_MILLIS: u64 = tag4(b"MTMS");
pub const SYS_TIME_MONOTONIC: u64 = tag4(b"MMON");
pub const SYS_TIME_RTC: u64 = tag4(b"MTRT");
pub const SYS_TIME_ADJUST: u64 = tag4(b"MTAD");
pub const SYS_BATTERY_STATUS: u64 = tag4(b"MBAT");
pub const SYS_PROC_STAT: u64 = tag4(b"MPST");
pub const SYS_PROC_OUTPUT: u64 = tag4(b"MOUT");
pub const SYS_PROC_INPUT: u64 = tag4(b"MPIN");
pub const SYS_STDIN_READ: u64 = tag4(b"MSRD");
