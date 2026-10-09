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


//! The CPU's own AES instructions, when it has them.
//!
//! AES-NI runs a round in one instruction, in constant time, with no table
//! for a cache to leak, and is a hundred times faster than the computed
//! S-box in sub_byte.rs, which stays as the path for a CPU without it. The
//! capsule target already enables SSE2, whose registers these instructions
//! use and the kernel saves. Support is read from CPUID once.
//!
//! The round keys are the same bytes the software key schedule writes: the
//! instructions take FIPS 197's expanded key in its own byte order.

use core::sync::atomic::{AtomicU8, Ordering};

const UNKNOWN: u8 = 0;
const PRESENT: u8 = 1;
const ABSENT: u8 = 2;
const FORCED_OFF: u8 = 3;

static STATE: AtomicU8 = AtomicU8::new(UNKNOWN);

/// Whether blocks are encrypted with the CPU's instructions.
pub fn in_use() -> bool {
    match STATE.load(Ordering::Relaxed) {
        PRESENT => true,
        ABSENT | FORCED_OFF => false,
        _ => {
            let present = detect();
            STATE.store(if present { PRESENT } else { ABSENT }, Ordering::Relaxed);
            present
        }
    }
}

/// Use the software cipher whatever the CPU has, or go back to detecting.
/// For proofs that hold both paths to the same answers.
pub fn force_software(on: bool) {
    STATE.store(if on { FORCED_OFF } else { UNKNOWN }, Ordering::Relaxed);
}

#[cfg(target_arch = "x86_64")]
fn detect() -> bool {
    // CPUID leaf 1, ECX bit 25: AES-NI.
    let leaf = core::arch::x86_64::__cpuid(1);
    leaf.ecx & (1 << 25) != 0
}

#[cfg(not(target_arch = "x86_64"))]
fn detect() -> bool {
    false
}

/// One block under `round_keys` (`rounds + 1` keys of 16 bytes) in place,
/// with the AES instructions. Only called once `in_use` has said yes. False
/// when the block was left alone, so the caller runs the software cipher
/// rather than passing plaintext on as if it were encrypted.
#[cfg(target_arch = "x86_64")]
pub(crate) fn encrypt(round_keys: &[u8], rounds: usize, block: &mut [u8; 16]) -> bool {
    if rounds == 0 || round_keys.len() < (rounds + 1) * 16 {
        return false;
    }
    // SAFETY: eK@nonos.systems. `in_use` read CPUID and found AES-NI, so the
    // instructions exist; every load and store is unaligned and within the
    // slices checked above.
    unsafe { encrypt_aesni(round_keys, rounds, block) };
    true
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn encrypt(_round_keys: &[u8], _rounds: usize, _block: &mut [u8; 16]) -> bool {
    false
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "aes,sse2")]
unsafe fn encrypt_aesni(round_keys: &[u8], rounds: usize, block: &mut [u8; 16]) {
    use core::arch::x86_64::{__m128i, _mm_aesenc_si128, _mm_aesenclast_si128, _mm_loadu_si128, _mm_storeu_si128, _mm_xor_si128};
    let key = |r: usize| _mm_loadu_si128(round_keys[r * 16..].as_ptr() as *const __m128i);
    let mut state = _mm_xor_si128(_mm_loadu_si128(block.as_ptr() as *const __m128i), key(0));
    for r in 1..rounds {
        state = _mm_aesenc_si128(state, key(r));
    }
    state = _mm_aesenclast_si128(state, key(rounds));
    _mm_storeu_si128(block.as_mut_ptr() as *mut __m128i, state);
}
