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

//! Host proofs for the TPM side of device enrollment. The kernel's own wire
//! code is included via `#[path]`, so the bytes under test are the bytes that
//! ship, and the transport under them is a software TPM 2.0 on TCP.
//!
//! What the byte tests cannot show is whether a real TPM takes each command,
//! whether the EK is the one tpm2-tools and the certificate name, and whether
//! a registrar's credential unwraps. The live tests show those.

extern crate alloc;

/// The loader handoff the device secret reads its approval from: none here.
#[cfg(test)]
pub mod boot;
/// The two kernel crypto entry points the included files call.
#[cfg(test)]
pub mod crypto;
/// The loader's rollback counter files, for the live R20 test.
#[cfg(test)]
pub mod loader_floor;
/// The kernel's module tree, enough of it for `crate::security::tpm::...` to
/// resolve to the shipping files.
#[cfg(test)]
pub mod security;

/// The kernel's `MkEnroll` decoding and dispatch, at the paths their imports
/// expect: everything but the copy to and from user memory.
#[cfg(test)]
pub mod syscall;

/// libc's framing of the same calls.
#[cfg(test)]
#[path = "../../libc/src/enroll/frame.rs"]
mod libc_enroll;
