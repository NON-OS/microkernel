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

//! What the bootloader recorded about the kernel it jumped to. Shown
//! before the disk is chosen, so a person installs a system whose verdict
//! they have read, in the same words the boot log used.

use nonos_libc::{mk_attest_status, AttestStatus};

pub struct Boot {
    pub available: bool,
    pub signature_ok: bool,
    pub secure_boot: bool,
    pub attested: bool,
    pub kernel_blake3: [u8; 32],
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
            kernel_blake3: s.kernel_blake3,
        }
    }

    /// The boot log's own words for the verdict.
    pub fn verdict(&self) -> &'static str {
        match (self.available, self.attested, self.signature_ok) {
            (false, _, _) => "attestation status unavailable",
            (true, true, _) => "kernel attested under the enrolled root",
            (true, false, true) => "kernel signature verified, not attested",
            (true, false, false) => "kernel not verified",
        }
    }
}
