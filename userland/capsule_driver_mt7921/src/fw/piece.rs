/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! One piece of the download: announced (`mt76_connac_mcu_init_download`:
//! address, length, mode, waited on), then its bytes sent bare.

use super::mode::request_for;
use super::Step;
use crate::dma::Dma;
use crate::hw::Regs;
use crate::mcu::Mcu;

/// Little-endian words, the bodies of the download commands.
pub fn words(w: &[u32]) -> [u8; 12] {
    let mut out = [0u8; 12];
    for (i, v) in w.iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    out
}

/// `bytes` bound for `at.0` in download mode `at.1`.
pub fn piece(m: &mut Mcu, r: &Regs, d: &mut Dma, at: (u32, u32), bytes: &[u8]) -> Result<(), Step> {
    let (addr, mode) = at;
    let req = words(&[addr, bytes.len() as u32, mode]);
    m.send(r, d, request_for(addr), &req, true).map_err(|_| Step::Request)?;
    m.scatter(r, d, bytes).map_err(|_| Step::Send)
}
