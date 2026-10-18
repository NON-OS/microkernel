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

//! NFSR v1, the sealed record: 240 bytes, little-endian, every byte public.
//! nonos_proof_abi reads it; seal_proofs holds the two to each other.

pub(crate) const RECORD_BYTES: usize = 240;

/// Sealed after the last guest, covering every call since the first.
pub(crate) const FINAL: u16 = 1;
/// Some count saturated: the counts are lower bounds only.
pub(crate) const SATURATED: u16 = 2;
/// More than MAX_MEAS distinct measurements: M stopped absorbing.
pub(crate) const OVERFLOW: u16 = 4;

pub(crate) struct Facts {
    pub(crate) flags: u16,
    pub(crate) sup_meas: [u8; 32],
    pub(crate) sup_caps: u64,
    pub(crate) authority: u8,
    pub(crate) seal_no: u64,
    pub(crate) trace: [u8; 32],
    pub(crate) rows: u64,
    pub(crate) hc: [u64; 4],
    pub(crate) meas: [u8; 32],
    pub(crate) n_meas: u32,
    pub(crate) app_id: [u8; 32],
}

/// Bytes 208 to 240 stay zero: the diagnostic is private.
pub(crate) fn encode(f: &Facts) -> [u8; RECORD_BYTES] {
    let mut b = [0u8; RECORD_BYTES];
    b[..4].copy_from_slice(b"NFSR");
    b[4..6].copy_from_slice(&1u16.to_le_bytes());
    b[6..8].copy_from_slice(&f.flags.to_le_bytes());
    b[8..40].copy_from_slice(&f.sup_meas);
    b[40..48].copy_from_slice(&f.sup_caps.to_le_bytes());
    b[48] = f.authority;
    b[56..64].copy_from_slice(&f.seal_no.to_le_bytes());
    b[64..96].copy_from_slice(&f.trace);
    b[96..104].copy_from_slice(&f.rows.to_le_bytes());
    for (i, w) in f.hc.iter().enumerate() {
        b[104 + 8 * i..112 + 8 * i].copy_from_slice(&w.to_le_bytes());
    }
    b[136..168].copy_from_slice(&f.meas);
    b[168..172].copy_from_slice(&f.n_meas.to_le_bytes());
    b[176..208].copy_from_slice(&f.app_id);
    b
}
