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

//! The boot attestation verdict the handoff carries to the kernel.

#[derive(Debug, Clone)]
pub struct BootAttestationResult {
    /// The kernel's measurement is enrolled under the boot root.
    pub verified: bool,
    /// The measurement the verdict is about, on both handoff fields.
    pub program_hash: [u8; 32],
    pub capsule_commitment: [u8; 32],
    pub status_message: &'static str,
}

impl Default for BootAttestationResult {
    fn default() -> Self {
        Self {
            verified: false,
            program_hash: [0u8; 32],
            capsule_commitment: [0u8; 32],
            status_message: "not verified",
        }
    }
}

impl BootAttestationResult {
    /// The kernel whose measurement is `kernel_hash` passed the gate.
    pub fn verified(kernel_hash: [u8; 32]) -> Self {
        Self {
            verified: true,
            program_hash: kernel_hash,
            capsule_commitment: kernel_hash,
            status_message: "kernel attestation verified",
        }
    }
}
