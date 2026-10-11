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

//! A family once sealed: its public record, and the witness only the prover
//! may read.

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::hist::HIST_WORDS;
use super::record::RECORD_BYTES;

pub(crate) struct Sealed {
    pub(crate) seal_no: u64,
    /// Slot 0, the app MkSealRead finds the record by.
    pub(crate) app_id: [u8; 32],
    pub(crate) record: [u8; RECORD_BYTES],
    pub(crate) hist: Box<[u32; HIST_WORDS]>,
    pub(crate) salt: [u64; 4],
    pub(crate) diag: [u8; 32],
    pub(crate) list: Vec<[u8; 32]>,
}

/// H, the salt and the diag.
pub(crate) const WITNESS_BYTES: usize = 4 * HIST_WORDS + 64;

/// MkSealRead's length for a list of `n` measurements, at most MAX_MEAS.
pub(crate) const fn len_for(n: usize) -> usize {
    RECORD_BYTES + WITNESS_BYTES + 32 * n
}

impl Sealed {
    /// H as 1026 little-endian u32s, the salt as four u64s, then the diag,
    /// written straight into `out`, so no other copy is left to clear.
    fn witness_into(&self, out: &mut Vec<u8>) {
        self.hist.iter().for_each(|c| out.extend_from_slice(&c.to_le_bytes()));
        self.salt.iter().for_each(|w| out.extend_from_slice(&w.to_le_bytes()));
        out.extend_from_slice(&self.diag);
    }

    pub(crate) fn bytes_len(&self) -> usize {
        len_for(self.list.len())
    }

    /// What MkSealRead hands over: record | witness | measurement list.
    pub(crate) fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.bytes_len());
        out.extend_from_slice(&self.record);
        self.witness_into(&mut out);
        self.list.iter().for_each(|m| out.extend_from_slice(m));
        out
    }
}
