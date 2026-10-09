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

//! Microkernel syscall ABI tags, mirroring `SyscallNumber::Mk*` for the numeric router.

use crate::syscall::abi::tag4;

pub const SYS_IPC_SEND: u64 = tag4(b"MISD");
pub const SYS_IPC_RECV: u64 = tag4(b"MIRC");
pub const SYS_IPC_CALL: u64 = tag4(b"MICL");
pub const SYS_IPC_RECV_FROM: u64 = tag4(b"MIRF");
pub const SYS_IPC_REPLY: u64 = tag4(b"MIRY");
pub const SYS_IPC_SEND_TO_PID: u64 = tag4(b"MISP");
pub const SYS_SERVICE_LOOKUP: u64 = tag4(b"MSVL");
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
/// Program stdout: the caller's own `proc.<pid>` inbox, never serial; needs only IPC.
pub const SYS_STDOUT_WRITE: u64 = tag4(b"MSOW");
/// Output only the caller's launcher reads: never serial, whatever the caps.
pub const SYS_PRIVATE_WRITE: u64 = tag4(b"MPVW");
/// The package store's sectors, written and read on the disk the block layer chose.
pub const SYS_STORE_WRITE: u64 = tag4(b"MSWR");
pub const SYS_STORE_READ: u64 = tag4(b"MSRR");
/// The data volume: a verified import, a file's size, a range of it, a passphrase.
pub const SYS_DATA_IMPORT: u64 = tag4(b"MDIM");
pub const SYS_DATA_STAT: u64 = tag4(b"MDST");
pub const SYS_DATA_READ: u64 = tag4(b"MDRD");
pub const SYS_DATA_PASSPHRASE: u64 = tag4(b"MDPW");
/// An import fed chunk by chunk, by a capsule holding StreamImport (model fetch, prove).
pub const SYS_DATA_FEED_BEGIN: u64 = tag4(b"MDFB");
pub const SYS_DATA_FEED: u64 = tag4(b"MDFD");
pub const SYS_DATA_REMOVE: u64 = tag4(b"MDRM");
pub const SYS_ATTEST_STATUS: u64 = tag4(b"MAST");
/// The roots, depths and epochs the gates check against, as a versioned record.
pub const SYS_ATTEST_POLICY: u64 = tag4(b"MAPY");
/// The kernel's verdict on the bootloader that started it, as a versioned record.
pub const SYS_BOOT_ATTEST: u64 = tag4(b"MBTA");
/// A signed attestation document; then the entries its registry root folds, to recompute it.
pub const SYS_ATTEST_DOC: u64 = tag4(b"MADC");
pub const SYS_ATTEST_ENTRIES: u64 = tag4(b"MAEN");
/// The last of what the kernel wrote to its serial console, kept in memory.
pub const SYS_LOG_TAIL: u64 = tag4(b"MLOG");
/// This machine's TPM-derived device secret, to the one capsule holding DeviceSecret.
pub const SYS_DEVICE_SECRET: u64 = tag4(b"MDVS");
/// The bootloader's and the kernel's slot of the device proof, to the same capsule.
pub const SYS_BOOT_SLOTS: u64 = tag4(b"MBSL");
/// Enrollment's TPM half: the EK, its certificate, the AK, activation, signing.
pub const SYS_ENROLL: u64 = tag4(b"MENR");
/// A chunk of the image this machine booted, for the installer to write.
pub const SYS_INSTALL_SOURCE: u64 = tag4(b"MISR");
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
/// Mint a trailer for something this machine is installing.
pub const SYS_LOCAL_SIGN: u64 = tag4(b"MLSG");
/// Ask whether an image is proved under a root this machine trusts.
pub const SYS_LOCAL_VERIFY: u64 = tag4(b"MLVF");
/// Ask for a distribution package to be installed.
pub const SYS_APP_INSTALL: u64 = tag4(b"MAIN");
/// Ask to enrol this machine's own build root.
pub const SYS_DEV_ROOT_LOCAL: u64 = tag4(b"MDRO");
/// Grant or withdraw consent to run what this machine installs.
pub const SYS_LOCAL_CONSENT: u64 = tag4(b"MLCG");
/// Restore that consent, at setup, from the token a grant returned.
pub const SYS_LOCAL_RESTORE: u64 = tag4(b"MLCR");
/// Start the program a distribution package installed.
pub const SYS_APP_LAUNCH: u64 = tag4(b"MAPL");
/// Where an asked-for install stands.
pub const SYS_APP_INSTALL_STATUS: u64 = tag4(b"MAIS");
pub const SYS_APP_UNINSTALL: u64 = tag4(b"MAUN");
/// Ask to enrol a signing root so software built here runs here. Prints a
/// confirmation code; enrols nothing on its own.
pub const SYS_DEV_ROOT_REQUEST: u64 = tag4(b"MDRQ");
/// Complete a pending enrolment with the code the kernel displayed.
pub const SYS_DEV_ROOT_CONFIRM: u64 = tag4(b"MDRC");
pub const SYS_CAP_GRANT: u64 = tag4(b"MCGT");
pub const SYS_CAP_REVOKE: u64 = tag4(b"MCRV");
pub const SYS_CAP_CHECK: u64 = tag4(b"MCCK");
pub const SYS_DEVICE_LIST: u64 = tag4(b"MDLS");
pub const SYS_DEVICE_CLAIM: u64 = tag4(b"MDCL");
pub const SYS_DEVICE_RELEASE: u64 = tag4(b"MDRL");
pub const SYS_MMIO_MAP: u64 = tag4(b"MMMP");
pub const SYS_MMIO_UNMAP: u64 = tag4(b"MMUM");
pub const SYS_IRQ_BIND: u64 = tag4(b"MIRB");
pub const SYS_IRQ_UNBIND: u64 = tag4(b"MIRU");
pub const SYS_IRQ_ACK: u64 = tag4(b"MIRA");
pub const SYS_IRQ_POLL: u64 = tag4(b"MIRP");
pub const SYS_IRQ_WAIT: u64 = tag4(b"MIRW");
pub const SYS_DMA_MAP: u64 = tag4(b"MDMM");
pub const SYS_DMA_UNMAP: u64 = tag4(b"MDMU");
pub const SYS_PIO_GRANT: u64 = tag4(b"MPGT");
pub const SYS_PIO_READ: u64 = tag4(b"MPRD");
pub const SYS_PIO_WRITE: u64 = tag4(b"MPWR");
pub const SYS_PIO_RELEASE: u64 = tag4(b"MPRL");
pub const SYS_MK_DEBUG: u64 = tag4(b"MDBG");
pub const SYS_PCI_CONFIG_READ: u64 = tag4(b"MPCR");
pub const SYS_PCI_CONFIG_WRITE: u64 = tag4(b"MPCW");
/// Spawn another window of an embedded, attested app (terminal or browser); needs SpawnWindow.
pub const SYS_SPAWN_INSTANCE: u64 = tag4(b"MSPI");
/// Run a baked, attested tool by name as the caller's child, to drive its stdio; needs IPC.
pub const SYS_TOOL_RUN: u64 = tag4(b"MTRN");
pub const SYS_TTY_SET: u64 = tag4(b"MTTY");
pub const SYS_TTY_QUERY: u64 = tag4(b"MTTQ");
