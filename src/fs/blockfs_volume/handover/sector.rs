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

//! The handover's writes of the key header's sector, each read back before
//! it counts, and the handover key, which opens a record.

use super::super::error::VolumeError;
use super::super::key_header_io::{read_key_sector, write_key_sector};
use super::super::say::say;
use super::layout::{put, Record};
use crate::crypto::constant_time::secure_zero;
use crate::hardware::block_device::BlockDeviceError;
use crate::security::tpm::device_secret::{from_boot, handover_key};

/// The handover key's label: another label, as well as another policy,
/// from way 3's wrapping key (key_pick.rs).
const LABEL: &[u8] = b"blockfs.data.handover.v1";

/// Write `s` and read it back: Ok only when the disk holds exactly `s`.
pub(super) fn write_checked(s: &[u8; 512]) -> Result<(), VolumeError> {
    write_key_sector(s)?;
    match read_key_sector()? == *s {
        true => Ok(()),
        false => Err(VolumeError::Device(BlockDeviceError::DeviceFailure)),
    }
}

/// The handover key, when this boot's chain is one the release approved.
pub(super) fn handover_kek() -> Option<[u8; 32]> {
    from_boot().and_then(|a| handover_key(&a, LABEL).ok())
}

/// The volume key the record `r` in `sector` holds, if the handover key
/// opens it.
pub(super) fn by_record(r: &Record, sector: &[u8; 512]) -> Option<[u8; 32]> {
    let mut hk = handover_kek()?;
    let key = super::seal::unwrap(&hk, r, sector);
    secure_zero(&mut hk);
    key
}

/// `sector` written without its record, said either way: a record left
/// behind is cleared again by the next boot.
pub(super) fn clear(sector: &[u8; 512]) {
    match write_checked(&put(sector, None)) {
        Ok(()) => say("[DATA] a handover this boot did not need is cleared"),
        Err(e) => crate::log::warn!("[DATA] a handover left in place: {:?}", e),
    }
}
