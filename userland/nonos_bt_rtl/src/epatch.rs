// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The "Realtech" epatch file (btrtl.h, struct rtl_epatch_header; btrtl.c,
//! rtlbt_parse_firmware): the 8-octet signature, the firmware version
//! (le32) and the patch count (le16); then, for that count, each patch's
//! chip id (le16), then each one's length (le16), then each one's offset
//! in the file (le32). The patch for a ROM is the one whose chip id is the
//! ROM version plus one. Its last four octets are sent as the firmware
//! version instead (`image`).

use crate::error::RtlError;
use crate::project::{project, EXTENSION_SIG, PROJECT_8821C};

pub const SIGNATURE: &[u8; 8] = b"Realtech";
pub const HEADER: usize = 14;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Epatch<'a> {
    pub fw_version: u32,
    /// The chosen patch, as the file holds it.
    pub patch: &'a [u8],
}

fn le16(fw: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([fw[at], fw[at + 1]])
}

fn le32(fw: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([fw[at], fw[at + 1], fw[at + 2], fw[at + 3]])
}

/// The RTL8821C patch for ROM version `rom`.
pub fn select(fw: &[u8], rom: u8) -> Result<Epatch<'_>, RtlError> {
    if fw.len() < HEADER + EXTENSION_SIG.len() + 3 {
        return Err(RtlError::Truncated);
    }
    if !fw.starts_with(SIGNATURE) {
        return Err(RtlError::Signature);
    }
    match project(fw)? {
        PROJECT_8821C => {}
        other => return Err(RtlError::Project(other)),
    }
    let count = usize::from(le16(fw, 12));
    if fw.len() < HEADER + 8 * count {
        return Err(RtlError::Truncated);
    }
    let (ids, lens, offsets) = (HEADER, HEADER + 2 * count, HEADER + 4 * count);
    let wanted = u16::from(rom) + 1;
    let i = (0..count).find(|i| le16(fw, ids + 2 * i) == wanted);
    let Some(i) = i else { return Err(RtlError::NoPatchFor(rom)) };
    let (len, at) = (usize::from(le16(fw, lens + 2 * i)), le32(fw, offsets + 4 * i) as usize);
    // btrtl.c takes an offset of 0 as no patch found.
    if at == 0 {
        return Err(RtlError::NoPatchFor(rom));
    }
    let patch = fw.get(at..at.saturating_add(len)).filter(|p| p.len() >= 4);
    let patch = patch.ok_or(RtlError::Truncated)?;
    Ok(Epatch { fw_version: le32(fw, 8), patch })
}
