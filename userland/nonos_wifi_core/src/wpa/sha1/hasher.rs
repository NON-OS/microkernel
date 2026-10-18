// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! SHA-1 (RFC 3174). Deprecated as a general hash, but WPA2 is defined over it:
//! HMAC-SHA1, PBKDF2 and the 802.11i PRF all build on this. Streaming, no_std,
//! no allocation. Checked against RFC known-answer vectors in `iwlwifi_proofs`.

pub struct Sha1 {
    pub(super) h: [u32; 5],
    pub(super) block: [u8; 64],
    pub(super) block_len: usize,
    pub(super) total_len: u64,
}

impl Sha1 {
    pub fn new() -> Self {
        Self {
            h: [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476, 0xC3D2_E1F0],
            block: [0u8; 64],
            block_len: 0,
            total_len: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        while !data.is_empty() {
            let n = core::cmp::min(64 - self.block_len, data.len());
            self.block[self.block_len..self.block_len + n].copy_from_slice(&data[..n]);
            self.block_len += n;
            data = &data[n..];
            if self.block_len == 64 {
                self.process();
                self.block_len = 0;
            }
        }
    }
}
