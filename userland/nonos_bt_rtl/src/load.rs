// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The patch loaded, before the host's bring-up (btrtl.c, btrtl_initialize
//! and btrtl_download_firmware). Each command is answered by Command
//! Complete with status 0 or the load stops: Read ROM Version (0xfc6d)
//! returns one octet, and each download fragment (0xfc20) one, its index.
//! The host's bring-up, next, begins with HCI Reset.

use alloc::vec::Vec;

use nonos_bt_hci::{Host, Transport};
use nonos_bt_wire::hci::opcode::{Opcode, READ_LOCAL_VERSION};

use crate::chip::{chip, version, Chip};
use crate::error::RtlError;
use crate::fragments::fragments;
use crate::ids::realtek;
use crate::image::image;

pub const READ_ROM_VERSION: Opcode = Opcode(0xfc6d);
pub const DOWNLOAD: Opcode = Opcode(0xfc20);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Not a Realtek USB id: nothing was sent.
    NotRealtek,
    /// A Realtek controller that is not an unpatched RTL8821C, or another
    /// company's chip on a Realtek id: left as it is.
    Running,
    /// The patch for ROM version `rom`, of firmware `fw_version`, was sent
    /// in `fragments` commands.
    Patched { rom: u8, fw_version: u32, fragments: usize },
}

/// Load the patch into the controller behind `host`, whose USB device
/// descriptor gave `ids` (vendor, product).
pub fn prepare<T: Transport>(
    host: &mut Host<T>,
    ids: (u16, u16),
    fw: &[u8],
    cfg: &[u8],
) -> Result<Outcome, RtlError> {
    if !realtek(ids.0, ids.1) {
        return Ok(Outcome::NotRealtek);
    }
    if chip(&version(&host.command(READ_LOCAL_VERSION, &[])?)?) != Chip::Rom8821c {
        return Ok(Outcome::Running);
    }
    let rom = match host.command(READ_ROM_VERSION, &[])?.as_slice() {
        &[rom] => rom,
        _ => return Err(RtlError::RomReply),
    };
    let image = image(fw, cfg, rom)?;
    let fw_version = u32::from_le_bytes([fw[8], fw[9], fw[10], fw[11]]);
    let mut sent = 0;
    for (index, data) in fragments(&image) {
        let mut p = Vec::with_capacity(1 + data.len());
        p.push(index);
        p.extend_from_slice(data);
        if host.command(DOWNLOAD, &p)?.len() != 1 {
            return Err(RtlError::DownloadReply);
        }
        sent += 1;
    }
    Ok(Outcome::Patched { rom, fw_version, fragments: sent })
}
