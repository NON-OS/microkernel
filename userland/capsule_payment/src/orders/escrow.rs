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

//! An order's escrow files. When ORDER_NEW's order is kept, it
//! takes its offer: the offer moves to /data/meter.escrow.<id> and its slot
//! goes. ORDER_DELIVERED's checked plaintext waits in /data/meter.plain.<id>
//! for op 18. Both leave with the order.

use alloc::vec::Vec;

use nonos_nox_meter::escrow::KeptOffer;
use nonos_nox_meter::orders::{order_takes, Order};
use nonos_payment_proto::{EIO, ENOENT};

use super::offers::{offers, path as slot_path};
use super::sealed::{load_sealed, save_sealed};
use crate::disk;

fn path(kind: &str, order_id: &[u8; 32]) -> Vec<u8> {
    let hex: alloc::string::String =
        order_id[..8].iter().map(|b| alloc::format!("{b:02x}")).collect();
    alloc::format!("/data/meter.{kind}.{hex}").into_bytes()
}

/// Move the offer the order's terms name to the order's own file.
pub fn bind(o: &Order, now: u64) -> Result<(), i32> {
    let live = offers(now)?;
    let (slot, kept) = live.iter().find(|(_, k)| order_takes(o, k)).ok_or(ENOENT)?;
    save_sealed(&path("escrow", &o.order_id), b"meter.escrow", kept.encode())?;
    disk::remove(&slot_path(*slot))
}

pub fn escrow_of(o: &Order) -> Result<KeptOffer, i32> {
    let mut plain = load_sealed(&path("escrow", &o.order_id), b"meter.escrow")?.ok_or(ENOENT)?;
    // The record holds the offer's buyer key: wiped once read, as offers.rs does.
    let kept = KeptOffer::decode(&plain);
    crate::wipe::wipe(&mut plain);
    kept.ok_or(EIO)
}

pub fn keep_plain(order_id: &[u8; 32], plain: Vec<u8>) -> Result<(), i32> {
    save_sealed(&path("plain", order_id), b"meter.plain", plain)
}

/// ENOENT before ORDER_DELIVERED's OK.
pub fn plain_of(order_id: &[u8; 32]) -> Result<Vec<u8>, i32> {
    load_sealed(&path("plain", order_id), b"meter.plain")?.ok_or(ENOENT)
}

pub fn forget(order_id: &[u8; 32]) -> Result<(), i32> {
    disk::remove(&path("escrow", order_id))?;
    disk::remove(&path("plain", order_id))
}
