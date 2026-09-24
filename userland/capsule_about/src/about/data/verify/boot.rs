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

//! Reading back the boot record.

use nonos_libc::{mk_attest_status, AttestStatus};

use super::types::{Recorded, Verdict};

// A recorded flag is a byte the bootloader wrote: 1 is the only value that means
// yes, and anything else is a failure rather than noise.
fn flag(byte: u8) -> Verdict {
    Verdict::from_bool(byte == 1)
}

/// `None` when the kernel will not answer, which the screen must render as
/// unknown rather than as the absence of a problem.
pub fn recorded() -> Option<Recorded> {
    let mut att = AttestStatus::default();
    if mk_attest_status(&mut att) != 0 {
        return None;
    }
    Some(Recorded {
        kernel_signature: flag(att.kernel_sig_ok),
        secure_boot: flag(att.secure_boot),
        attestation: flag(att.zk_attestation_ok),
        proof: flag(att.zk_verified),
        kernel_hash: att.kernel_blake3,
        program_hash: att.program_hash,
    })
}
