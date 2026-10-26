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

//! The Mk* syscalls, part 2 of 4, in the registry's order (mk.rs).

use super::mk::e;
use crate::syscall::abi::AbiEntry;
use crate::syscall::numbers::SyscallNumber;

pub(super) const ENTRIES: &[AbiEntry] = &[
    e(b"MBSL", SyscallNumber::MkBootSlots, "MkBootSlots"),
    e(b"MENR", SyscallNumber::MkEnroll, "MkEnroll"),
    e(b"MISR", SyscallNumber::MkInstallSource, "MkInstallSource"),
    e(b"MDRQ", SyscallNumber::MkDevRootRequest, "MkDevRootRequest"),
    e(b"MDRC", SyscallNumber::MkDevRootConfirm, "MkDevRootConfirm"),
    e(b"MCGT", SyscallNumber::MkCapGrant, "MkCapGrant"),
    e(b"MCRV", SyscallNumber::MkCapRevoke, "MkCapRevoke"),
    e(b"MCCK", SyscallNumber::MkCapCheck, "MkCapCheck"),
    e(b"MDLS", SyscallNumber::MkDeviceList, "MkDeviceList"),
    e(b"MDCL", SyscallNumber::MkDeviceClaim, "MkDeviceClaim"),
    e(b"MDRL", SyscallNumber::MkDeviceRelease, "MkDeviceRelease"),
    e(b"MBFB", SyscallNumber::MkBootFramebuffer, "MkBootFramebuffer"),
    e(b"MMMP", SyscallNumber::MkMmioMap, "MkMmioMap"),
    e(b"MMUM", SyscallNumber::MkMmioUnmap, "MkMmioUnmap"),
    e(b"MIRB", SyscallNumber::MkIrqBind, "MkIrqBind"),
    e(b"MIRU", SyscallNumber::MkIrqUnbind, "MkIrqUnbind"),
    e(b"MIRA", SyscallNumber::MkIrqAck, "MkIrqAck"),
    e(b"MIRP", SyscallNumber::MkIrqPoll, "MkIrqPoll"),
    e(b"MIRW", SyscallNumber::MkIrqWait, "MkIrqWait"),
    e(b"MDMM", SyscallNumber::MkDmaMap, "MkDmaMap"),
    e(b"MDMU", SyscallNumber::MkDmaUnmap, "MkDmaUnmap"),
    e(b"MPCR", SyscallNumber::MkPciConfigRead, "MkPciConfigRead"),
    e(b"MPCW", SyscallNumber::MkPciConfigWrite, "MkPciConfigWrite"),
    e(b"MPGT", SyscallNumber::MkPioGrant, "MkPioGrant"),
    e(b"MPRD", SyscallNumber::MkPioRead, "MkPioRead"),
    e(b"MPWR", SyscallNumber::MkPioWrite, "MkPioWrite"),
    e(b"MPRL", SyscallNumber::MkPioRelease, "MkPioRelease"),
    e(b"MHVB", SyscallNumber::MkHvBackdoor, "MkHvBackdoor"),
    e(b"MHVP", SyscallNumber::MkHvPost, "MkHvPost"),
    e(b"MHVR", SyscallNumber::MkHvRecv, "MkHvRecv"),
    e(b"MHVE", SyscallNumber::MkHvEvents, "MkHvEvents"),
    e(b"MHVS", SyscallNumber::MkHvSignal, "MkHvSignal"),
    e(b"MHVC", SyscallNumber::MkHvChannel, "MkHvChannel"),
    e(b"MDBG", SyscallNumber::MkDebug, "MkDebug"),
    e(b"MSRG", SyscallNumber::MkSurfaceRegister, "MkSurfaceRegister"),
    e(b"MSSH", SyscallNumber::MkSurfaceShare, "MkSurfaceShare"),
    e(b"MSAT", SyscallNumber::MkSurfaceAttach, "MkSurfaceAttach"),
    e(b"MSRL", SyscallNumber::MkSurfaceRelease, "MkSurfaceRelease"),
    e(b"MSPR", SyscallNumber::MkSurfacePresent, "MkSurfacePresent"),
    e(b"MDVW", SyscallNumber::MkDisplayVsyncWait, "MkDisplayVsyncWait"),
    e(b"MIEP", SyscallNumber::MkInputEventPost, "MkInputEventPost"),
];
