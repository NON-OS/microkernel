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

//! Preparing for an update, MkDataHandover (mod.rs): the open volume's key
//! sealed once more under the handover key into the key header's sector.
//! The installer asks it on the person's word, right before they start the
//! next release's stick. Only a boot whose own chain opens the volume
//! prepares, so the key sealed is the one way 3 wraps, never a key held in
//! memory for a volume of another kind.

use super::super::error::VolumeError;
use super::super::key_header::{parse_key_header, Keyed};
use super::super::key_header_io::read_key_sector;
use super::super::key_pick::opens_to;
use super::super::state::VOLUME;
use super::layout::put;
use super::sector::{handover_kek, write_checked};
use super::{seal, HandoverRefusal as R};
use crate::crypto::constant_time::secure_zero;
use crate::crypto::rng::fill_random_bytes_secure;

/// Write a handover for the volume open now, read back. Nothing is written
/// on a refusal.
pub fn prepare_handover() -> Result<(), VolumeError> {
    if !crate::security::zerostate::keeping() {
        return Err(VolumeError::Handover(R::NotKept));
    }
    let Some(mut key) = VOLUME.read().as_ref().map(|v| v.key) else {
        return Err(VolumeError::Handover(R::NotOpen));
    };
    let result = prepare_for(&key);
    secure_zero(&mut key);
    result
}

fn prepare_for(key: &[u8; 32]) -> Result<(), VolumeError> {
    let sector = read_key_sector()?;
    let wrapped = match parse_key_header(&sector) {
        Ok(Some(Keyed::Approved(w))) => w,
        Ok(Some(Keyed::Passphrase(_))) => return Err(VolumeError::Handover(R::NoNeed)),
        _ => return Err(VolumeError::Handover(R::CannotKeep)),
    };
    if !opens_to(&wrapped, key) {
        return Err(VolumeError::Handover(R::NotApproved));
    }
    let Some(mut hk) = handover_kek() else {
        return Err(VolumeError::Handover(R::NotApproved));
    };
    let mut nonce = [0u8; 12];
    let drawn = fill_random_bytes_secure(&mut nonce).is_ok();
    let record = seal::wrap(&hk, key, nonce, &sector).filter(|_| drawn);
    let written = record.map(|r| put(&sector, Some(&r)));
    let sound = written.zip(record).is_some_and(|(s, r)| seal::unwrap(&hk, &r, &s) == Some(*key));
    secure_zero(&mut hk);
    match written.filter(|_| sound) {
        Some(s) => write_checked(&s),
        None => Err(VolumeError::NoRandomness),
    }
}
