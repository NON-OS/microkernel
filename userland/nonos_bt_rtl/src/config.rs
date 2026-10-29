// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The config file sent after the patch (btrtl.c, btrtl_setup_rtl8723b:
//! the config is appended to the patch and the two are downloaded as one).
//! It begins with the signature 0x8723ab55 (le32) and the length of what
//! follows (le16), then entries of an offset (le16), a length (u8) and that
//! many octets (btrtl.h, struct rtl_vendor_config). btrtl.c sends it as it
//! is on USB; this host also checks that the entries fill it exactly, so a
//! cut or padded file is refused rather than sent.

use crate::error::RtlError;

pub const CONFIG_SIG: u32 = 0x8723_ab55;
const HEAD: usize = 6;
const ENTRY_HEAD: usize = 3;

pub fn check(cfg: &[u8]) -> Result<(), RtlError> {
    let (Some(sig), Some(len)) = (cfg.get(..4), cfg.get(4..HEAD)) else {
        return Err(RtlError::Config);
    };
    let sig = u32::from_le_bytes([sig[0], sig[1], sig[2], sig[3]]);
    let len = usize::from(u16::from_le_bytes([len[0], len[1]]));
    if sig != CONFIG_SIG || cfg.len() != HEAD + len {
        return Err(RtlError::Config);
    }
    let mut at = HEAD;
    while at < cfg.len() {
        let n = *cfg.get(at + 2).ok_or(RtlError::Config)?;
        at += ENTRY_HEAD + usize::from(n);
    }
    if at != cfg.len() {
        return Err(RtlError::Config);
    }
    Ok(())
}
