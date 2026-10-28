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

//! Intel Read Version (0xfc05, parameter 0xff) in its TLV form, read as
//! `btintel_parse_version_tlv` reads it: after the status, a type, a
//! length and that many octets, again to the end. Only what the download
//! needs is kept: the OTP Bluetooth address (type 0x30) is passed over.

use alloc::vec::Vec;

/// The image running: the bootloader, operational firmware, or between
/// them (0x02) an intermediate loader, Blazar's (btintel.h BTINTEL_IMG_*).
pub const IMG_BOOTLOADER: u8 = 0x01;
pub const IMG_OP: u8 = 0x03;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Version {
    pub cnvi_top: u32,
    pub cnvr_top: u32,
    pub cnvi_bt: u32,
    pub img_type: u8,
    /// Build number, week and year: the oldest the bootloader takes, or
    /// what operational firmware runs. Later TLVs overwrite earlier ones.
    pub build: [u8; 3],
    /// The secure boot engine: 0 RSA, 1 ECDSA.
    pub sbe_type: u8,
    pub fw_id: Vec<u8>,
}

impl Version {
    pub fn platform(&self) -> u8 {
        (self.cnvi_bt >> 8) as u8
    }
    pub fn variant(&self) -> u8 {
        ((self.cnvi_bt >> 16) & 0x3f) as u8
    }
}

/// `ret` is what follows the status. `None` if a TLV runs past the end, a
/// value is shorter than its field, or the firmware id is not a name.
pub fn parse(mut ret: &[u8]) -> Option<Version> {
    let mut v = Version::default();
    while let [kind, len, rest @ ..] = ret {
        let val = rest.get(..usize::from(*len))?;
        let le32 = || val.get(..4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
        match kind {
            0x10 => v.cnvi_top = le32()?,
            0x11 => v.cnvr_top = le32()?,
            0x12 => v.cnvi_bt = le32()?,
            0x1c => v.img_type = *val.first()?,
            0x1d => v.build[1..].copy_from_slice(val.get(..2)?),
            0x1f => v.build[0] = *val.first()?,
            0x2d => v.build.copy_from_slice(val.get(..3)?),
            0x2f => v.sbe_type = *val.first()?,
            0x50 => v.fw_id = super::name::fw_id(val)?,
            _ => {}
        }
        ret = &rest[val.len()..];
    }
    ret.is_empty().then_some(v)
}
