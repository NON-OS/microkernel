// NONOS Operating System (AGPL-3.0-or-later)

//! The key schedule and the block cipher, forward and inverse.

use super::forward::{add_round_key, mix_columns, shift_rows, sub_bytes};
use super::inverse::{inv_mix_columns, inv_shift_rows, inv_sub_bytes};
use super::tables::{RCON, SBOX};

/// An expanded AES-128 key: eleven 16-byte round keys.
pub struct Aes128 {
    round_keys: [u8; 176],
}

impl Aes128 {
    /// Expand a 16-byte key into the round-key schedule.
    pub fn new(key: &[u8; 16]) -> Self {
        let mut w = [0u8; 176];
        w[..16].copy_from_slice(key);
        let mut i = 16;
        let mut rc = 0;
        while i < 176 {
            let mut t = [w[i - 4], w[i - 3], w[i - 2], w[i - 1]];
            if i % 16 == 0 {
                t = [
                    SBOX[t[1] as usize] ^ RCON[rc],
                    SBOX[t[2] as usize],
                    SBOX[t[3] as usize],
                    SBOX[t[0] as usize],
                ];
                rc += 1;
            }
            for j in 0..4 {
                w[i + j] = w[i - 16 + j] ^ t[j];
            }
            i += 4;
        }
        Self { round_keys: w }
    }

    /// Encrypt one 16-byte block in place with the forward cipher.
    pub fn encrypt_block(&self, block: &mut [u8; 16]) {
        add_round_key(block, &self.round_keys[..16]);
        for round in 1..10 {
            sub_bytes(block);
            shift_rows(block);
            mix_columns(block);
            add_round_key(block, &self.round_keys[round * 16..round * 16 + 16]);
        }
        sub_bytes(block);
        shift_rows(block);
        add_round_key(block, &self.round_keys[160..176]);
    }
}

impl Aes128 {
    /// Decrypt one 16-byte block in place with the inverse cipher.
    pub fn decrypt_block(&self, block: &mut [u8; 16]) {
        add_round_key(block, &self.round_keys[160..176]);
        for round in (1..10).rev() {
            inv_shift_rows(block);
            inv_sub_bytes(block);
            add_round_key(block, &self.round_keys[round * 16..round * 16 + 16]);
            inv_mix_columns(block);
        }
        inv_shift_rows(block);
        inv_sub_bytes(block);
        add_round_key(block, &self.round_keys[..16]);
    }
}
