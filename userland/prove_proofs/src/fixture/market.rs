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

//! A sealed family as MkSealRead hands it to nonos.prove, for an app whose
//! policy allows read, write, exit and exit_group under LISTING: forty reads,
//! three refused writes, a refused ptrace outside the set, an exit_group.

use nonos_proof_abi::trust::LINUX_CAPS;
use nonos_proof_abi::{app_id, hist_commit, meas_chain, Record, FLAG_FINAL, HIST_WORDS};

pub const LISTING: [u8; 32] = [0x4C; 32];
pub const PROGRAM: [u8; 32] = [0xA7; 32];
pub const SUP_MEAS: [u8; 32] = [0x5C; 32];
pub const SALT: [u64; 4] = [11, 22, 33, 44];

pub fn policy(listing: &[u8; 32], allowed: &[usize]) -> [u8; 256] {
    let mut p = [0u8; 256];
    p[..8].copy_from_slice(b"NLXPOL01");
    p[8..10].copy_from_slice(&1u16.to_le_bytes());
    p[16..48].copy_from_slice(&PROGRAM);
    p[48..56].copy_from_slice(&0x21u64.to_le_bytes());
    p[56..88].copy_from_slice(listing);
    allowed.iter().for_each(|&b| p[152 + b / 8] |= 1 << (b % 8));
    p
}

pub fn hist() -> [u32; HIST_WORDS] {
    let mut h = [0u32; HIST_WORDS];
    (h[0], h[3], h[2 * 231], h[2 * 101 + 1]) = (40, 3, 1, 2);
    h
}

pub fn record(policy: &[u8; 256], h: &[u32; HIST_WORDS], meas: &[[u8; 32]]) -> Record {
    Record {
        flags: FLAG_FINAL,
        sup_meas: SUP_MEAS,
        sup_caps: LINUX_CAPS,
        authority: 0,
        seal_no: 0x7A11_5EA1,
        trace: [0x7E; 32],
        rows: 47,
        hc: hist_commit(h, &SALT).expect("salt below p"),
        meas: meas_chain(meas),
        n_meas: meas.len() as u32,
        app_id: app_id(policy, &PROGRAM),
    }
}

/// record | H | salt | diag | list, MkSealRead's bytes.
pub fn sealed_bytes(rec: &Record, h: &[u32; HIST_WORDS], meas: &[[u8; 32]]) -> Vec<u8> {
    let mut b = rec.encode().to_vec();
    h.iter().for_each(|c| b.extend_from_slice(&c.to_le_bytes()));
    SALT.iter().for_each(|w| b.extend_from_slice(&w.to_le_bytes()));
    b.extend_from_slice(&[0xD1; 32]);
    meas.iter().for_each(|m| b.extend_from_slice(m));
    b
}
