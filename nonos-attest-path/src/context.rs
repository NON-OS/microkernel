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

//! The contexts every slot is enrolled under, built by one function each, so
//! the enroll tool and every gate read and write the same bytes. A gate builds
//! the context from what it is about to run; it never takes one from a trailer.

/// A capsule: its measurement, the capability word it is granted, the policy
/// epoch, as 32 + 8 + 8 bytes, big-endian.
pub fn capsule_context(measurement: &[u8; 32], caps: u64, epoch: u64) -> [u8; 48] {
    let mut c = [0u8; 48];
    c[..32].copy_from_slice(measurement);
    c[32..40].copy_from_slice(&caps.to_be_bytes());
    c[40..48].copy_from_slice(&epoch.to_be_bytes());
    c
}

/// A kernel or a bootloader: its measurement and the boot epoch, 32 + 8 bytes.
/// The two share the shape and the leaf's kind keeps them apart. A kernel's
/// measurement is BLAKE3 of its image; a bootloader's is the PE Authenticode
/// SHA-256 the firmware records in the TPM log, so the kernel can rebuild it.
pub fn boot_context(measurement: &[u8; 32], epoch: u64) -> [u8; 40] {
    let mut c = [0u8; 40];
    c[..32].copy_from_slice(measurement);
    c[32..40].copy_from_slice(&epoch.to_be_bytes());
    c
}
