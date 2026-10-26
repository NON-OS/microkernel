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

//! Output, the package store, the data volume, attestation, the log and enrolment.

use crate::syscall::abi::tag4;

/// Program stdout: the caller's own `proc.<pid>` inbox, never serial; needs only IPC.
pub const SYS_STDOUT_WRITE: u64 = tag4(b"MSOW");
/// Output only the caller's launcher reads: never serial, whatever the caps.
pub const SYS_PRIVATE_WRITE: u64 = tag4(b"MPVW");
/// The package store's sectors, written and read on the disk the block layer chose.
pub const SYS_STORE_WRITE: u64 = tag4(b"MSWR");
pub const SYS_STORE_READ: u64 = tag4(b"MSRR");
/// Whether this boot keeps anything: 1 once the person chose it, else 0.
pub const SYS_KEEPING: u64 = tag4(b"MKEP");
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
