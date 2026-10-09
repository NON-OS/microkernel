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

//! What the bootloader recorded about the kernel it jumped to, and what the
//! kernel checked after it: the bootloader, and every capsule at spawn.
//! Shown before the disk is chosen, so a person installs a system whose
//! proofs they have read, in the same words the boot log used.

use nonos_libc::boot_attest::{boot_attest, BootAttest};
use nonos_libc::{mk_attest_policy, mk_attest_status, AttestPolicy, AttestStatus, ZK_PATH_ONLY};

use super::census::{census, Census};
use super::tpm::Tpm;

/// Each reading the kernel may decline is an `Option`, read as unknown.
pub struct Boot {
    pub available: bool,
    pub signature_ok: bool,
    pub secure_boot: bool,
    pub attested: bool,
    pub proof: bool,
    /// A development boot: the kernel and every capsule were checked on
    /// their Merkle paths alone, with no STARK proof.
    pub path_only: bool,
    pub kernel_blake3: [u8; 32],
    pub program_hash: [u8; 32],
    pub tpm: Tpm,
    pub policy: Option<AttestPolicy>,
    pub loader: Option<BootAttest>,
    pub capsules: Option<Census>,
}

impl Boot {
    pub fn read() -> Boot {
        let mut s = AttestStatus::default();
        let rc = mk_attest_status(&mut s as *mut AttestStatus);
        Boot {
            available: rc >= 0,
            signature_ok: s.kernel_sig_ok != 0,
            secure_boot: s.secure_boot != 0,
            attested: s.zk_attestation_ok != 0,
            proof: s.zk_verified != 0,
            path_only: s.zk_verified == ZK_PATH_ONLY,
            kernel_blake3: s.kernel_blake3,
            program_hash: s.program_hash,
            tpm: Tpm::read(),
            policy: mk_attest_policy(),
            loader: boot_attest(),
            capsules: census(),
        }
    }

    /// The boot log's own words for the verdict.
    pub fn verdict(&self) -> &'static str {
        match (self.available, self.attested, self.signature_ok) {
            (false, _, _) => "attestation status unavailable",
            (true, true, _) if self.path_only => "kernel path checked, development: no STARK",
            (true, true, _) => "kernel attested under the enrolled root",
            (true, false, true) => "kernel signature verified, not attested",
            (true, false, false) => "kernel not verified",
        }
    }
}
