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

//! Sending: append the MMIE to a group-addressed management frame and fill
//! its MIC (IEEE Std 802.11-2020, 12.5.4.5). The caller owns the IPN and
//! must hand a larger one to every frame under the same key.

use alloc::vec::Vec;

use super::error::BipError;
use super::key::BipGmacKey;
use super::mic::{header_len, mic};
use super::mmie::{zeroed, IPN_MAX, MIC_AT};

pub fn protect(key: &BipGmacKey, ipn: u64, frame: &mut Vec<u8>) -> Result<(), BipError> {
    let header = header_len(frame).ok_or(BipError::NotManagement)?;
    if frame[4] & 1 == 0 {
        return Err(BipError::NotGroupAddressed);
    }
    if ipn == 0 || ipn > IPN_MAX {
        return Err(BipError::BadIpn);
    }
    let at = frame.len() + MIC_AT;
    frame.extend_from_slice(&zeroed(key.id(), ipn));
    let m = mic(key.bytes(), frame, header, ipn);
    frame[at..].copy_from_slice(&m);
    Ok(())
}
