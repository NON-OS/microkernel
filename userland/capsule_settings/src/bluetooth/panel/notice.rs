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

//! What the page says a request did, in words: the status words are NBTC
//! v1's (letter E-25).

use nonos_bt_proto::ops::*;

use super::reach::Reach;
use super::request::Request;

pub fn done(req: &Request) -> &'static str {
    match req.op {
        OP_SCAN if req.body() == [1] => "Scanning for keyboards and mice",
        // The capsule ends a look it has begun, at most 10 s on (decision
        // 6 of letter E-25), so the page does not claim it stopped at once.
        OP_SCAN => "Scanning stops when the current look ends",
        OP_PAIR => "A new keyboard or mouse may pair in the next minute",
        OP_FORGET => "Forgotten. Pairing it again needs a new pairing",
        OP_ANSWER => "Answer sent",
        _ => "Done",
    }
}

pub fn refused(status: i32) -> &'static str {
    match status {
        E_NOENT => "That device is no longer paired",
        E_IO => "The key store refused the change",
        E_ACCES => "This window may not change Bluetooth",
        E_BUSY => "Bluetooth is busy. Try again",
        E_INVAL => "Bluetooth did not take that",
        _ => "Bluetooth refused",
    }
}

pub fn unreached(reach: Reach) -> &'static str {
    match reach {
        Reach::NoService | Reach::Unasked => "Bluetooth is not running",
        Reach::NoAnswer | Reach::Answered => "Bluetooth is not answering",
    }
}
