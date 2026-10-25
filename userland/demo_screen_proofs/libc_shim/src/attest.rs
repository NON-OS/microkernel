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

//! The attestation calls: every process admitted under a proof, the boot's
//! checks passed, and no signed document on the host (no TPM to sign it).

use crate::machine::PROCS;

pub const ATTEST_ENTRY_LEN: usize = 45;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AttestStatus {
    pub zk_verified: u8,
    pub kernel_sig_ok: u8,
    pub secure_boot: u8,
    pub zk_attestation_ok: u8,
    pub kernel_blake3: [u8; 32],
    pub program_hash: [u8; 32],
}

pub fn mk_attest_status(out: *mut AttestStatus) -> i64 {
    let kernel_blake3 = core::array::from_fn(|i| (i as u8).wrapping_mul(53) ^ 0x1F);
    let program_hash = core::array::from_fn(|i| (i as u8).wrapping_mul(29) ^ 0xE4);
    let s = AttestStatus {
        zk_verified: 1,
        kernel_sig_ok: 1,
        secure_boot: 1,
        zk_attestation_ok: 1,
        kernel_blake3,
        program_hash,
    };
    // SAFETY: the caller hands a writable AttestStatus, as the real call requires.
    unsafe { out.write(s) };
    0
}

/// Pid big-endian, the measurement, the mask big-endian, the authority (0:
/// admitted under a proof).
pub fn mk_attest_entries(out: &mut [u8]) -> i64 {
    let rows = PROCS.iter().zip(out.chunks_exact_mut(ATTEST_ENTRY_LEN));
    let mut n = 0;
    for (&(pid, _, mask), e) in rows {
        e[0..4].copy_from_slice(&pid.to_be_bytes());
        e[4..36]
            .iter_mut()
            .enumerate()
            .for_each(|(i, b)| *b = (i as u8 ^ pid as u8).wrapping_mul(13));
        e[36..44].copy_from_slice(&mask.to_be_bytes());
        e[44] = 0;
        n += ATTEST_ENTRY_LEN;
    }
    n as i64
}

pub fn mk_attest_doc(_challenge: &[u8; 32], _out: &mut [u8]) -> i64 {
    -19
}
