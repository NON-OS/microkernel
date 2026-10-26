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
use crate::syscall::abi::{tag4, AbiDomain, AbiEntry, AbiStatus};
use crate::syscall::numbers::SyscallNumber;
/// Every Mk* syscall. Most route to the microkernel dispatch; the surface and
/// input calls go to `surface_ops` and `input_ops` (router/dispatch_fn.rs).
pub(super) const ENTRIES: &[AbiEntry] = &[
    e(b"MISD", SyscallNumber::MkIpcSend, "MkIpcSend"),
    e(b"MIRC", SyscallNumber::MkIpcRecv, "MkIpcRecv"),
    e(b"MICL", SyscallNumber::MkIpcCall, "MkIpcCall"),
    e(b"MIRF", SyscallNumber::MkIpcRecvFrom, "MkIpcRecvFrom"),
    e(b"MIRY", SyscallNumber::MkIpcReply, "MkIpcReply"),
    e(b"MIRQ", SyscallNumber::MkIpcRecvReq, "MkIpcRecvReq"),
    e(b"MIRT", SyscallNumber::MkIpcReplyTo, "MkIpcReplyTo"),
    e(b"MISP", SyscallNumber::MkIpcSendToPid, "MkIpcSendToPid"),
    e(b"MSVL", SyscallNumber::MkServiceLookup, "MkServiceLookup"),
    e(b"MTGR", SyscallNumber::MkThreadGroup, "MkThreadGroup"),
    e(b"MSVR", SyscallNumber::MkServiceRegister, "MkServiceRegister"),
    e(b"MMAP", SyscallNumber::MkMmap, "MkMmap"),
    e(b"MUMP", SyscallNumber::MkMunmap, "MkMunmap"),
    e(b"MCLD", SyscallNumber::MkCapsuleLoad, "MkCapsuleLoad"),
    e(b"MEXT", SyscallNumber::MkExit, "MkExit"),
    e(b"MPAL", SyscallNumber::MkPidAlive, "MkPidAlive"),
    e(b"MWAT", SyscallNumber::MkWait, "MkWait"),
    e(b"MKIL", SyscallNumber::MkKill, "MkKill"),
    e(b"MGPD", SyscallNumber::MkGetPid, "MkGetPid"),
    e(b"MKAR", SyscallNumber::MkArgs, "MkArgs"),
    e(b"MTSP", SyscallNumber::MkThreadSpawn, "MkThreadSpawn"),
    e(b"MSTB", SyscallNumber::MkSetTls, "MkSetTls"),
    e(b"MYLD", SyscallNumber::MkYield, "MkYield"),
    e(b"MFTW", SyscallNumber::MkFutexWait, "MkFutexWait"),
    e(b"MFTK", SyscallNumber::MkFutexWake, "MkFutexWake"),
    e(b"MTMS", SyscallNumber::MkTimeMillis, "MkTimeMillis"),
    e(b"MMON", SyscallNumber::MkTimeMonotonic, "MkTimeMonotonic"),
    e(b"MTRT", SyscallNumber::MkTimeRtc, "MkTimeRtc"),
    e(b"MTAD", SyscallNumber::MkTimeAdjust, "MkTimeAdjust"),
    e(b"MBAT", SyscallNumber::MkBatteryStatus, "MkBatteryStatus"),
    e(b"MPST", SyscallNumber::MkProcStat, "MkProcStat"),
    e(b"MOUT", SyscallNumber::MkProcOutput, "MkProcOutput"),
    e(b"MPIN", SyscallNumber::MkProcInput, "MkProcInput"),
    e(b"MSRD", SyscallNumber::MkStdinRead, "MkStdinRead"),
    e(b"MAST", SyscallNumber::MkAttestStatus, "MkAttestStatus"),
    e(b"MADC", SyscallNumber::MkAttestDoc, "MkAttestDoc"),
    e(b"MAEN", SyscallNumber::MkAttestEntries, "MkAttestEntries"),
    e(b"MLOG", SyscallNumber::MkLogTail, "MkLogTail"),
    e(b"MAPY", SyscallNumber::MkAttestPolicy, "MkAttestPolicy"),
    e(b"MBTA", SyscallNumber::MkBootAttest, "MkBootAttest"),
    e(b"MDVS", SyscallNumber::MkDeviceSecret, "MkDeviceSecret"),
];
/// One Mk* entry, routed; the other Mk tables (mk2.rs to mk4.rs) build theirs with it.
pub(super) const fn e(tag: &[u8; 4], variant: SyscallNumber, name: &'static str) -> AbiEntry {
    AbiEntry { id: tag4(tag), variant, name, domain: AbiDomain::Mk, status: AbiStatus::Routed }
}
