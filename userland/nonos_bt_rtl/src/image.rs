// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! What is downloaded: the chosen patch with its last four octets replaced
//! by the file's firmware version (le32), then the config, whole (btrtl.c,
//! rtlbt_parse_firmware and btrtl_setup_rtl8723b).

use alloc::vec::Vec;

use crate::config::check;
use crate::epatch::select;
use crate::error::RtlError;

pub fn image(fw: &[u8], cfg: &[u8], rom: u8) -> Result<Vec<u8>, RtlError> {
    let e = select(fw, rom)?;
    check(cfg)?;
    let body = &e.patch[..e.patch.len() - 4];
    let mut out = Vec::with_capacity(e.patch.len() + cfg.len());
    out.extend_from_slice(body);
    out.extend_from_slice(&e.fw_version.to_le_bytes());
    out.extend_from_slice(cfg);
    Ok(out)
}
