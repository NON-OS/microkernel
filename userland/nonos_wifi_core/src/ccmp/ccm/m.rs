// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! AES-128 in CCM mode with an 8-byte tag and a 2-byte length field, exactly
//! as WPA2 CCMP protects a data frame: CBC-MAC over the additional data and the
//! payload for authentication, then CTR-mode encryption. Checked against the
//! RFC 3610 test vector in `nonos_wifi_core_proofs`. no_std, streaming, no allocation.

use super::*;

pub(super) const M: usize = 8; // tag length

pub(super) const L: usize = 2; // length-field octets; nonce is 15 - L = 13 bytes

pub(super) struct CbcMac<'a> {
    pub(super) aes: &'a Aes128,
    pub(super) x: [u8; 16],
    pub(super) fill: usize,
}

impl<'a> CbcMac<'a> {
    pub(super) fn new(aes: &'a Aes128) -> Self {
        Self { aes, x: [0u8; 16], fill: 0 }
    }
    pub(super) fn update(&mut self, data: &[u8]) {
        for &b in data {
            self.x[self.fill] ^= b;
            self.fill += 1;
            if self.fill == 16 {
                self.aes.encrypt_block(&mut self.x);
                self.fill = 0;
            }
        }
    }
    pub(super) fn close_block(&mut self) {
        if self.fill > 0 {
            self.aes.encrypt_block(&mut self.x);
            self.fill = 0;
        }
    }
}

pub(super) fn ctr_block(nonce: &[u8; 13], counter: u16) -> [u8; 16] {
    let mut a = [0u8; 16];
    a[0] = (L - 1) as u8;
    a[1..14].copy_from_slice(nonce);
    a[14..16].copy_from_slice(&counter.to_be_bytes());
    a
}

/// Compute the CCM tag over the additional data and the payload (in the clear).
pub(super) fn mac(aes: &Aes128, nonce: &[u8; 13], aad: &[u8], data: &[u8]) -> [u8; M] {
    let mut cbc = CbcMac::new(aes);
    let mut b0 = [0u8; 16];
    b0[0] = (if aad.is_empty() { 0 } else { 0x40 }) | ((((M - 2) / 2) as u8) << 3) | (L - 1) as u8;
    b0[1..14].copy_from_slice(nonce);
    b0[14..16].copy_from_slice(&(data.len() as u16).to_be_bytes());
    cbc.update(&b0);
    if !aad.is_empty() {
        cbc.update(&(aad.len() as u16).to_be_bytes());
        cbc.update(aad);
        cbc.close_block();
    }
    cbc.update(data);
    cbc.close_block();
    let mut t = [0u8; M];
    t.copy_from_slice(&cbc.x[..M]);
    t
}
