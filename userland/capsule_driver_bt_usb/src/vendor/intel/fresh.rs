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

//! A TLV part, as `btintel_bootloader_setup_tlv` loads it: its firmware
//! sent and booted, and the part asked again. If what runs then is an
//! intermediate loader (Blazar and later), a BlazarI B0 is given its DSBR,
//! and the firmware the loader names is sent and booted too. The DDC is the
//! last file sent's, named from the version that chose it, as Linux names it.

use super::name::stem;
use super::steps::{finish, load, running};
use super::tlv::{self, Version, IMG_OP};
use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;

pub const READ_VERSION: u16 = 0xfc05;
const IMG_IML: u8 = 0x02;
const DSBR: u16 = 0xfc0a;
const BLAZARI: u32 = 0x900;

pub fn fresh<P: UsbPipes>(w: &mut Wire<P>, mut v: Version) -> Result<(), HciError> {
    if v.img_type == IMG_OP {
        return running(w, &stem(&v, true), v.variant(), v.build);
    }
    let mut sent = stem(&v, true);
    for _ in 0..2 {
        load(w, &sent, v.variant(), v.sbe_type)?;
        let ret = w.command(READ_VERSION, &[0xff])?;
        v = version(w, &ret)?;
        match v.img_type {
            IMG_OP => return finish(w, &sent),
            IMG_IML => dsbr(w, &v)?,
            _ => break,
        }
        sent = stem(&v, true);
    }
    Err(w.refuse(&[b"firmware booted but is not operational"]))
}

/// Intel DSBR (0xfc0a), enable then value, for a BlazarI B0 running its
/// loader (`btintel_set_dsbr`). Linux reads both from the UEFI variable
/// UefiCnvCommonDSBR and sends zero without it; this capsule reads no UEFI
/// variables, so it sends zero, and a refusal ends the setup as in Linux.
fn dsbr<P: UsbPipes>(w: &mut Wire<P>, v: &Version) -> Result<(), HciError> {
    if v.cnvi_top & 0xfff == BLAZARI && (v.cnvi_top >> 24) & 0xf == 0x01 {
        w.command(DSBR, &[0x00, 0x00])?;
    }
    Ok(())
}

pub fn version<P: UsbPipes>(w: &Wire<P>, ret: &[u8]) -> Result<Version, HciError> {
    match tlv::parse(ret) {
        Some(v) if v.platform() == 0x37 => Ok(v),
        Some(_) => Err(w.refuse(&[b"not an Intel Bluetooth platform"])),
        None => Err(w.refuse(&[b"version malformed"])),
    }
}
