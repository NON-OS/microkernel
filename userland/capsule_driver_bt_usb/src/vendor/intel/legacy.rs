// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The parts before TyP: JfP, ThP, HrP (the AX201's Bluetooth) and CcP
//! (the AX200's). Read Version answers them in `struct intel_version`, ten
//! octets with the status, platform 0x37 second; firmware is named
//! ibt-<variant>-<hw revision>-<fw revision> in decimal
//! (`btintel_get_fw_name`); a booted part reports firmware variant 0x23.
//! Their bootloader is read with Read Boot Params (0xfc0d) first
//! (`btintel_download_fw`), whose OTP address is not kept.

use alloc::format;
use alloc::string::String;

use super::steps::{finish, load, running};
use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;

const OPERATIONAL: u8 = 0x23;
const READ_BOOT_PARAMS: u16 = 0xfc0d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Legacy {
    pub variant: u8,
    pub hw_revision: u8,
    pub fw_variant: u8,
    pub fw_revision: u8,
    /// Build number, week and year of what runs.
    pub build: [u8; 3],
}

/// `ret` is what follows the status.
pub fn parse(ret: &[u8]) -> Option<Legacy> {
    let &[0x37, variant, hw_revision, fw_variant, fw_revision, n, w, y, _patch] = ret else {
        return None;
    };
    Some(Legacy { variant, hw_revision, fw_variant, fw_revision, build: [n, w, y] })
}

pub fn stem(l: &Legacy) -> Option<String> {
    let named = matches!(l.variant, 0x11..=0x14);
    named.then(|| format!("ibt-{}-{}-{}", l.variant, l.hw_revision, l.fw_revision))
}

pub fn run<P: UsbPipes>(w: &mut Wire<P>, l: Legacy) -> Result<(), HciError> {
    let Some(stem) = stem(&l) else { return Err(w.refuse(&[b"hardware variant unknown to this loader"])) };
    if l.fw_variant == OPERATIONAL {
        return running(w, &stem, l.variant, l.build);
    }
    // 22 octets after the status, Limited_CCE at 20: zero when every
    // fragment is answered with Command Complete, as this loader needs.
    let p = w.command(READ_BOOT_PARAMS, &[])?;
    if p.len() != 22 || p[20] != 0 {
        return Err(w.refuse(&[b"bootloader does not answer every fragment"]));
    }
    load(w, &stem, l.variant, 0)?;
    finish(w, &stem)
}
