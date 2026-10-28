/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A MediaTek controller made ready for HCI, in Linux v6.12
//! `btmtk_usb_setup`'s order for the MT7921 and MT7922: the chip's id, ROM
//! version and flavour read, its patch fetched and checked (before the
//! chip is touched, so a medium without the package leaves it as it was),
//! the subsystem reset, the download, the endpoint reset option, and WMT
//! function control with 1, which turns the Bluetooth function on. Its
//! answer's status is not read, as Linux does not.

use super::chip::{chip, Chip};
use super::fail::Fail;
use super::fetch::fetch;
use super::files::{for_chip, File};
use super::load::{download, Sent};
use super::port::Port;
use super::reg::*;
use super::reset::reset;
use super::wmt::{wmt, OP_FUNC_CTRL};

pub struct Up {
    pub chip: Chip,
    pub sent: Sent,
}

/// `files` are the package's pins: `FILES` in the capsule.
pub fn setup(p: &mut impl Port, files: &[File; 2]) -> Result<Up, Fail> {
    let id = read(p, DEV_ID)?;
    let version = read(p, FW_VERSION)?;
    let chip = chip(id, version, read(p, FW_FLAVOR)?)?;
    let img = fetch(p, for_chip(files, chip))?;
    reset(p, chip)?;
    let sent = download(p, &img)?;
    uhw_write(p, EP_RST_OPT, EP_RST_IN_OUT)?;
    wmt(p, OP_FUNC_CTRL, 0, &[1])?;
    Ok(Up { chip, sent })
}
