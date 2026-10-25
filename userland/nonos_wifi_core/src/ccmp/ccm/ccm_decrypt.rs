// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Decrypt `input` (ciphertext followed by the 8-byte tag) and verify the
//! tag over `aad`.

use super::*;

/// Decrypt `input` (ciphertext followed by the 8-byte tag) and verify the tag
/// over `aad`. Writes the plaintext into `out` and returns its length, or
/// `None` if the tag does not match. The tag compare is constant time.
pub fn ccm_decrypt(
    key: &[u8; 16],
    nonce: &[u8; 13],
    aad: &[u8],
    input: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    if input.len() < M || aad.len() >= 0xFF00 {
        return None;
    }
    let clen = input.len() - M;
    if out.len() < clen {
        return None;
    }
    let aes = Aes128::new(key);
    apply_ctr(&aes, nonce, &input[..clen], &mut out[..clen]);
    let t = mac(&aes, nonce, aad, &out[..clen]);
    let mut s0 = ctr_block(nonce, 0);
    aes.encrypt_block(&mut s0);
    let mut diff = 0u8;
    for i in 0..M {
        diff |= input[clen + i] ^ (t[i] ^ s0[i]);
    }
    if diff == 0 {
        Some(clen)
    } else {
        None
    }
}
