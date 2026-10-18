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

//! The meter's escrow orders on disk (plan/21, v1.4.20), sealed since each
//! holds a refund salt. A file that does not open is EIO, never an empty
//! table: starting one afresh would lose the salts already kept.

use alloc::vec::Vec;

use nonos_nox_meter::orders::{decode_orders, encode_orders, Order, MAX_ORDERS, ORDERS_LEN};
use nonos_payment_proto::{EAGAIN, EINVAL, EIO, ENFILE, ENOENT};
use nonos_vault::OVERHEAD;

use crate::disk::{self, Loaded, ORDERS_PATH};
use crate::keys::{open_record, seal_record};
use crate::wipe::wipe;

const RECORD: &[u8] = b"meter.orders";

pub fn orders() -> Result<Vec<Order>, i32> {
    match disk::load(ORDERS_PATH, ORDERS_LEN + OVERHEAD) {
        Loaded::Silent => Err(EAGAIN),
        Loaded::Absent => Ok(Vec::new()),
        Loaded::Bytes(b) => {
            let mut plain = open_record(RECORD, &b)?;
            let all = decode_orders(&plain).ok_or(EIO);
            wipe(&mut plain);
            all
        }
    }
}

pub fn find(order_id: &[u8; 32]) -> Result<Order, i32> {
    orders()?.into_iter().find(|o| o.order_id == *order_id).ok_or(ENOENT)
}

/// Keep `o`, in place of any order with its id; ENFILE when the table is full.
pub fn keep(o: &Order) -> Result<(), i32> {
    let mut all = orders()?;
    all.retain(|x| x.order_id != o.order_id);
    if all.len() >= MAX_ORDERS {
        return Err(ENFILE);
    }
    all.push(*o);
    write(&all)
}

/// Let the order go, its escrow files with it: ORDER_STATUS found nothing
/// of it left to refund.
pub fn drop_order(order_id: &[u8; 32]) -> Result<(), i32> {
    let mut all = orders()?;
    all.retain(|x| x.order_id != *order_id);
    write(&all)?;
    super::escrow::forget(order_id)
}

fn write(all: &[Order]) -> Result<(), i32> {
    let mut plain = encode_orders(all).ok_or(EINVAL)?;
    let sealed = seal_record(RECORD, &plain);
    wipe(&mut plain);
    disk::save(ORDERS_PATH, &sealed?)
}
