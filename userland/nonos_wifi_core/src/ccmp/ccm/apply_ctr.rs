// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Counter-mode keystream and the sealing direction of CCM.

use super::*;

pub(super) fn apply_ctr(aes: &Aes128, nonce: &[u8; 13], input: &[u8], out: &mut [u8]) {
    let mut off = 0;
    let mut counter: u16 = 1;
    while off < input.len() {
        let mut s = ctr_block(nonce, counter);
        aes.encrypt_block(&mut s);
        let n = core::cmp::min(16, input.len() - off);
        for j in 0..n {
            out[off + j] = input[off + j] ^ s[j];
        }
        off += n;
        counter = counter.wrapping_add(1);
    }
}

/// Authenticate `aad` and encrypt `plaintext`, writing the ciphertext followed
/// by the 8-byte tag into `out`. Returns the written length.
pub fn ccm_encrypt(
    key: &[u8; 16],
    nonce: &[u8; 13],
    aad: &[u8],
    plaintext: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    if plaintext.len() > 0xFFFF || aad.len() >= 0xFF00 || out.len() < plaintext.len() + M {
        return None;
    }
    let aes = Aes128::new(key);
    let t = mac(&aes, nonce, aad, plaintext);
    apply_ctr(&aes, nonce, plaintext, &mut out[..plaintext.len()]);
    let mut s0 = ctr_block(nonce, 0);
    aes.encrypt_block(&mut s0);
    for i in 0..M {
        out[plaintext.len() + i] = t[i] ^ s0[i];
    }
    Some(plaintext.len() + M)
}
