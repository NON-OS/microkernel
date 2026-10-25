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

//! One NBTC request answered. Pure over the board, so the proofs drive it;
//! the capsule's desk only receives, finds the caller and replies.

use super::board::Board;
use super::bonds::{bonds, forget};
use super::look::{battery, health, link};
use super::ops::*;
use super::user::{answer as user_answer, passkey};
use super::who::{may, Caller};
use super::window::{pair, scan};
use super::wire::{decode, reply, status};
use nonos_bt_bond::Keys;

/// The reply to the request in `buf` from `who`, written to `tx`, which
/// holds `REPLY_MAX` octets; its length.
pub fn answer<K: Keys>(
    b: &mut Board<K>,
    who: Caller,
    buf: &[u8],
    now_ms: u64,
    tx: &mut [u8],
) -> usize {
    let (req, body) = match decode(buf) {
        Ok(r) => r,
        Err(req) => return status(tx, &req, E_INVAL),
    };
    if !may(who, req.op) {
        return status(tx, &req, E_ACCES);
    }
    let bare = body.is_empty();
    let data = match req.op {
        OP_HEALTH if bare => health(b, tx),
        OP_LINK if bare => link(b, now_ms, tx),
        OP_BATTERY if bare => battery(b, tx),
        OP_BONDS if bare => bonds(b, tx),
        OP_PASSKEY if bare => passkey(b, tx),
        OP_FORGET => return status(tx, &req, forget(b, body)),
        OP_PAIR if bare => return status(tx, &req, pair(b, now_ms)),
        OP_ANSWER => return status(tx, &req, user_answer(b, body)),
        OP_SCAN => return status(tx, &req, scan(b, body)),
        _ => return status(tx, &req, E_INVAL),
    };
    reply(tx, &req, E_OK, data)
}

/// The reply when the board is in use: the request is refused, not lost,
/// so its caller may ask again.
pub fn busy(buf: &[u8], tx: &mut [u8]) -> usize {
    let req = decode(buf).map_or_else(|r| r, |(r, _)| r);
    status(tx, &req, E_BUSY)
}
