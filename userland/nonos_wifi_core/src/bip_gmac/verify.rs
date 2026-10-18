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

//! Receiving: check a group-addressed management frame's MMIE (IEEE Std
//! 802.11-2020, 12.5.4.6). The IPN must be above the last one accepted under
//! the key and the MIC must match; on success the frame's IPN is returned
//! for the caller to record, so a frame that fails changes nothing.

use alloc::vec::Vec;

use super::error::BipError;
use super::key::BipGmacKey;
use super::mic::{header_len, mic};
use super::mmie::{read, MIC_AT, MIC_LEN, MMIE_LEN};

pub fn verify(key: &BipGmacKey, frame: &[u8], last_ipn: u64) -> Result<u64, BipError> {
    let header = header_len(frame).ok_or(BipError::NotManagement)?;
    if frame[4] & 1 == 0 {
        return Err(BipError::NotGroupAddressed);
    }
    if frame.len() < header + MMIE_LEN {
        return Err(BipError::NoMmie);
    }
    let e = read(&frame[header..]).ok_or(BipError::NoMmie)?;
    if e.key_id != key.id() {
        return Err(BipError::WrongKey);
    }
    if e.ipn <= last_ipn {
        return Err(BipError::Replay);
    }
    let mut zeroed: Vec<u8> = frame.to_vec();
    let at = frame.len() - MMIE_LEN + MIC_AT;
    zeroed[at..].fill(0);
    let want = mic(key.bytes(), &zeroed, header, e.ipn);
    // Constant time, so a forger learns nothing from how long a guess took.
    let diff = (0..MIC_LEN).fold(0u8, |d, i| d | (want[i] ^ e.mic[i]));
    if diff != 0 {
        return Err(BipError::BadMic);
    }
    Ok(e.ipn)
}
