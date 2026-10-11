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

//! The kept escrow offers (plan/29): /data/meter.offer.N, sealed length u64
//! LE then the sealed record. Past OFFER_TTL or unopenable counts as none (no
//! money rests on an untaken offer); a new one takes a free slot, else the oldest.

use alloc::vec::Vec;

use nonos_nox_config::MAX_OFFERS;
use nonos_nox_meter::escrow::KeptOffer;
use nonos_payment_proto::EAGAIN;

use crate::disk::{self, Loaded};
use crate::keys::{open_record, seal_record};
use crate::wipe::wipe;

const RECORD: &[u8] = b"meter.offer";

pub fn path(slot: usize) -> Vec<u8> {
    alloc::format!("/data/meter.offer.{slot}").into_bytes()
}

fn read(slot: usize) -> Result<Option<KeptOffer>, i32> {
    let head = match disk::load(&path(slot), 8) {
        Loaded::Silent => return Err(EAGAIN),
        Loaded::Absent => return Ok(None),
        Loaded::Bytes(b) => b,
    };
    let n = u64::from_le_bytes(head[..8].try_into().map_err(|_| EAGAIN)?) as usize;
    let Loaded::Bytes(b) = disk::load(&path(slot), 8usize.saturating_add(n)) else {
        return Ok(None);
    };
    let Ok(mut plain) = open_record(RECORD, &b[8..]) else { return Ok(None) };
    let kept = KeptOffer::decode(&plain);
    wipe(&mut plain);
    Ok(kept)
}

/// Every live offer, with its slot.
pub fn offers(now: u64) -> Result<Vec<(usize, KeptOffer)>, i32> {
    let mut out = Vec::new();
    for slot in 0..MAX_OFFERS {
        if let Some(k) = read(slot)?.filter(|k| !k.expired(now)) {
            out.push((slot, k));
        }
    }
    Ok(out)
}

pub fn keep_offer(k: &KeptOffer, now: u64) -> Result<(), i32> {
    let live = offers(now)?;
    let free = (0..MAX_OFFERS).find(|s| live.iter().all(|(t, _)| t != s));
    let oldest = live.iter().min_by_key(|(_, o)| o.made_at).map(|(s, _)| *s);
    let slot = free.or(oldest).ok_or(EAGAIN)?;
    let mut plain = k.encode();
    let sealed = seal_record(RECORD, &plain);
    wipe(&mut plain);
    let sealed = sealed?;
    let file = [&(sealed.len() as u64).to_le_bytes()[..], &sealed].concat();
    disk::save(&path(slot), &file)
}
