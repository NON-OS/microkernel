// NONOS Operating System (AGPL-3.0-or-later)

//! The key schedule and the block cipher, forward and inverse.

use super::forward::{add_round_key, mix_columns, shift_rows, sub_bytes};
use super::inverse::{inv_mix_columns, inv_shift_rows, inv_sub_bytes};
use super::schedule::expand;
use crate::wipe::wipe;

/// An expanded AES-128 key: eleven 16-byte round keys.
pub struct Aes128 {
    round_keys: [u8; 176],
}

impl Aes128 {
    /// Expand a 16-byte key into the round-key schedule.
    pub fn new(key: &[u8; 16]) -> Self {
        Self { round_keys: expand(key) }
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

impl Drop for Aes128 {
    fn drop(&mut self) {
        wipe(&mut self.round_keys);
    }
}
