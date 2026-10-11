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

//! AES-256 encryption for GCMP-256 (FIPS 197): the Nk = 8 key expansion
//! of section 5.2, which applies SubWord without RotWord at every fourth
//! word of each eight, and fourteen rounds. The round operations are the
//! AES-128 ones in `ccmp::aes`; only the schedule and round count differ.

use crate::ccmp::aes::{add_round_key, mix_columns, shift_rows, sub_bytes, RCON, SBOX};

/// Fifteen 16-byte round keys.
pub struct Aes256 {
    rk: [u8; 240],
}

impl Aes256 {
    pub fn new(key: &[u8; 32]) -> Self {
        let mut w = [0u8; 240];
        w[..32].copy_from_slice(key);
        for i in (32..240).step_by(4) {
            let mut t = [w[i - 4], w[i - 3], w[i - 2], w[i - 1]];
            if i % 32 == 0 {
                t = [
                    SBOX[t[1] as usize] ^ RCON[i / 32 - 1],
                    SBOX[t[2] as usize],
                    SBOX[t[3] as usize],
                    SBOX[t[0] as usize],
                ];
            } else if i % 32 == 16 {
                t = t.map(|b| SBOX[b as usize]);
            }
            for j in 0..4 {
                w[i + j] = w[i - 32 + j] ^ t[j];
            }
        }
        Self { rk: w }
    }

    pub fn encrypt_block(&self, block: &mut [u8; 16]) {
        add_round_key(block, &self.rk[..16]);
        for round in 1..14 {
            sub_bytes(block);
            shift_rows(block);
            mix_columns(block);
            add_round_key(block, &self.rk[round * 16..round * 16 + 16]);
        }
        sub_bytes(block);
        shift_rows(block);
        add_round_key(block, &self.rk[224..240]);
    }
}

impl Drop for Aes256 {
    /// The schedule is the key; it does not outlive its use.
    fn drop(&mut self) {
        for b in self.rk.iter_mut() {
            // SAFETY: a valid, aligned, exclusively borrowed byte.
            unsafe { core::ptr::write_volatile(b, 0) };
        }
    }
}
