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

//! Sealed runs, the prover, local signing, installs and build roots.

use crate::syscall::abi::tag4;

/// Take the newest final record sealed for an app, with its witness and
/// measurements; it is dropped as it is read.
pub const SYS_SEAL_READ: u64 = tag4(b"MSLR");
/// A supervisor's note in its family's record: the app, or a diagnostic.
pub const SYS_SEAL_NOTE: u64 = tag4(b"MSNT");
/// Ask for the headless prover, app.prove.job: PROVE for the store, DELIVER
/// for a vendor supervisor of a live family.
pub const SYS_PROVE_JOB: u64 = tag4(b"MPJB");
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
