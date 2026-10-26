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

//! The Mk* syscalls, part 3 of 4, in the registry's order (mk.rs).

use super::mk::e;
use crate::syscall::abi::AbiEntry;
use crate::syscall::numbers::SyscallNumber;

pub(super) const ENTRIES: &[AbiEntry] = &[
    e(b"MIED", SyscallNumber::MkInputEventDrain, "MkInputEventDrain"),
    e(b"MIEW", SyscallNumber::MkInputEventWait, "MkInputEventWait"),
    e(b"MSPI", SyscallNumber::MkSpawnInstance, "MkSpawnInstance"),
    e(b"MFSP", SyscallNumber::MkForeignSpawn, "MkForeignSpawn"),
    e(b"MFST", SyscallNumber::MkForeignStart, "MkForeignStart"),
    e(b"MFWT", SyscallNumber::MkForeignWait, "MkForeignWait"),
    e(b"MFRP", SyscallNumber::MkForeignReply, "MkForeignReply"),
    e(b"MFCX", SyscallNumber::MkForeignContext, "MkForeignContext"),
    e(b"MFSG", SyscallNumber::MkForeignSignal, "MkForeignSignal"),
    e(b"MFIN", SyscallNumber::MkForeignInterrupt, "MkForeignInterrupt"),
    e(b"MPMP", SyscallNumber::MkPeerMap, "MkPeerMap"),
    e(b"MPCP", SyscallNumber::MkPeerCopy, "MkPeerCopy"),
    e(b"MPPT", SyscallNumber::MkPeerProtect, "MkPeerProtect"),
    e(b"MFTH", SyscallNumber::MkForeignThread, "MkForeignThread"),
    e(b"MPTL", SyscallNumber::MkPeerTls, "MkPeerTls"),
    e(b"MFFK", SyscallNumber::MkForeignFork, "MkForeignFork"),
    e(b"MPUN", SyscallNumber::MkPeerUnmap, "MkPeerUnmap"),
    e(b"MFEX", SyscallNumber::MkForeignExec, "MkForeignExec"),
    e(b"MLRN", SyscallNumber::MkLinuxRunNet, "MkLinuxRunNet"),
    e(b"MSLR", SyscallNumber::MkSealRead, "MkSealRead"),
    e(b"MSNT", SyscallNumber::MkSealNote, "MkSealNote"),
    e(b"MPJB", SyscallNumber::MkProveJob, "MkProveJob"),
    e(b"MLSG", SyscallNumber::MkLocalSign, "MkLocalSign"),
    e(b"MLVF", SyscallNumber::MkLocalVerify, "MkLocalVerify"),
    e(b"MAIN", SyscallNumber::MkAppInstall, "MkAppInstall"),
    e(b"MDRO", SyscallNumber::MkDevRootLocal, "MkDevRootLocal"),
    e(b"MLCG", SyscallNumber::MkLocalConsent, "MkLocalConsent"),
    e(b"MLCR", SyscallNumber::MkLocalRestore, "MkLocalRestore"),
    e(b"MAPL", SyscallNumber::MkAppLaunch, "MkAppLaunch"),
    e(b"MAIS", SyscallNumber::MkAppInstallStatus, "MkAppInstallStatus"),
    e(b"MAUN", SyscallNumber::MkAppUninstall, "MkAppUninstall"),
    e(b"MTRN", SyscallNumber::MkToolRun, "MkToolRun"),
    e(b"MCOF", SyscallNumber::MkCapsOf, "MkCapsOf"),
    e(b"MTTY", SyscallNumber::MkTtySet, "MkTtySet"),
    e(b"MTTQ", SyscallNumber::MkTtyQuery, "MkTtyQuery"),
    e(b"MSOW", SyscallNumber::MkStdoutWrite, "MkStdoutWrite"),
    e(b"MPVW", SyscallNumber::MkPrivateWrite, "MkPrivateWrite"),
    e(b"MSWR", SyscallNumber::MkStoreWrite, "MkStoreWrite"),
    e(b"MSRR", SyscallNumber::MkStoreRead, "MkStoreRead"),
    e(b"MKEP", SyscallNumber::MkKeeping, "MkKeeping"),
    e(b"MDIM", SyscallNumber::MkDataImport, "MkDataImport"),
];
