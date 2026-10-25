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

//! What MkSealRead hands nonos.prove (plan/10): record[240] | witness | the
//! measurement list, 32 bytes each. The witness is H (1026 u32), the salt
//! (4 u64) and the diag[32], all little-endian. Nothing else is accepted.

use alloc::vec::Vec;

use nonos_proof_abi::{decode_record, Record, HIST_WORDS, RECORD_BYTES};

pub const WITNESS_BYTES: usize = 4 * HIST_WORDS + 32 + 32;

pub struct Sealed {
    pub raw: [u8; RECORD_BYTES],
    pub record: Record,
    pub hist: [u32; HIST_WORDS],
    pub salt: [u64; 4],
    pub list: Vec<[u8; 32]>,
}

/// The parts of `b`, or `None` for a record that does not decode or a list
/// that is not exactly the record's n_meas entries.
pub fn split(b: &[u8]) -> Option<Sealed> {
    let raw: [u8; RECORD_BYTES] = b.get(..RECORD_BYTES)?.try_into().ok()?;
    let record = decode_record(&raw)?;
    let w = b.get(RECORD_BYTES..RECORD_BYTES + WITNESS_BYTES)?;
    let rest = &b[RECORD_BYTES + WITNESS_BYTES..];
    if rest.len() != 32 * record.n_meas as usize {
        return None;
    }
    let word = |at: usize, n: usize| w.get(at..at + n);
    let mut hist = [0u32; HIST_WORDS];
    for (i, h) in hist.iter_mut().enumerate() {
        *h = u32::from_le_bytes(word(4 * i, 4)?.try_into().ok()?);
    }
    let at = 4 * HIST_WORDS;
    let mut salt = [0u64; 4];
    for (i, s) in salt.iter_mut().enumerate() {
        *s = u64::from_le_bytes(word(at + 8 * i, 8)?.try_into().ok()?);
    }
    let list = rest
        .chunks_exact(32)
        .map(|c| {
            let mut m = [0u8; 32];
            m.copy_from_slice(c);
            m
        })
        .collect();
    Some(Sealed { raw, record, hist, salt, list })
}

/// H and the salt are the private witness: cleared, not left to the allocator.
impl Drop for Sealed {
    fn drop(&mut self) {
        // SAFETY: valid, aligned fields of `self`, borrowed mutably.
        self.hist.iter_mut().for_each(|h| unsafe { core::ptr::write_volatile(h, 0) });
        self.salt.iter_mut().for_each(|s| unsafe { core::ptr::write_volatile(s, 0) });
    }
}
