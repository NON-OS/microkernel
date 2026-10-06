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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Measurements {
    pub kernel_blake3: [u8; 32],
    pub kernel_sig_ok: u8,
    pub secure_boot: u8,
    pub zk_attestation_ok: u8,
    pub reserved: [u8; 5],
}

impl Default for Measurements {
    fn default() -> Self {
        Self {
            kernel_blake3: [0; 32],
            kernel_sig_ok: 0,
            secure_boot: 0,
            zk_attestation_ok: 0,
            reserved: [0; 5],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ZkAttestation {
    pub verified: u8,
    pub flags: u8,
    pub reserved: [u8; 6],
    pub program_hash: [u8; 32],
    pub capsule_commitment: [u8; 32],
}

impl Default for ZkAttestation {
    fn default() -> Self {
        Self {
            verified: 0,
            flags: 0,
            reserved: [0; 6],
            program_hash: [0; 32],
            capsule_commitment: [0; 32],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RngSeed {
    pub seed32: [u8; 32],
}

impl Default for RngSeed {
    fn default() -> Self {
        Self { seed32: [0; 32] }
    }
}

/// The policy the boot chain checked this kernel against. `kernel_root` is the
/// enrolled root the bootloader's path gate folded to, and is zero with
/// `checked` clear unless that gate ran and passed on this boot.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AttestPolicy {
    pub kernel_root: [u8; 32],
    pub boot_epoch: u64,
    pub depth: u8,
    pub checked: u8,
    /// One when `approval` holds the release's approval file for this kernel.
    pub approval_present: u8,
    pub reserved: [u8; 5],
    /// The approval file as the bootloader read it: key x, y, then r, s. Only
    /// its signature is trusted, and only under the key compiled into this
    /// kernel.
    pub approval: [u8; 128],
}

/// The kernel reads this at the same offsets the bootloader writes it; the
/// handoff version guards the rest, this guards the block itself.
const _: () = assert!(core::mem::size_of::<AttestPolicy>() == 176);

impl Default for AttestPolicy {
    fn default() -> Self {
        Self {
            kernel_root: [0; 32],
            boot_epoch: 0,
            depth: 0,
            checked: 0,
            approval_present: 0,
            reserved: [0; 5],
            approval: [0; 128],
        }
    }
}
