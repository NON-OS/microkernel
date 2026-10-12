// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Expand a 16-byte key into the round-key schedule.

use super::*;

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

pub(crate) fn add_round_key(s: &mut [u8; 16], k: &[u8]) {
    for (b, kb) in s.iter_mut().zip(k.iter()) {
        *b ^= *kb;
    }
}

pub(crate) fn sub_bytes(s: &mut [u8; 16]) {
    for b in s.iter_mut() {
        *b = SBOX[*b as usize];
    }
}

pub(crate) fn shift_rows(s: &mut [u8; 16]) {
    // state is column-major: s[r + 4c]. row r rotates left by r.
    let t = *s;
    for r in 1..4 {
        for c in 0..4 {
            s[r + 4 * c] = t[r + 4 * ((c + r) % 4)];
        }
    }
}
