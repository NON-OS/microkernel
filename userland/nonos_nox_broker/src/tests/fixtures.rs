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

//! The issuance decision end to end: a receipt in, a grant out, once.

use nonos_nox_config::{address, Contract, MAINNET};
use nonos_nox_license::{price_of, ToolId};
use nonos_nox_receipt::TRANSFER_TOPIC;

use crate::issue::{PaidTo, Terms};

pub const BUYER: [u8; 20] = [
    0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x11, 0x22, 0x33,
];

/// The listing's payee and the live NOX token, as the meter would pass them.
pub fn to() -> PaidTo {
    let token = address(MAINNET, Contract::NoxToken).expect("NOX on mainnet");
    PaidTo { payee: [0x5a; 20], token }
}

/// Terms with every field zero: no expiry, no device.
pub fn terms() -> Terms {
    Terms { now: 0, ttl_secs: 0, nonce: [0; 8], device: [0; 32] }
}

fn hex_of(bytes: &[u8]) -> String {
    let mut s = String::from("0x");
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0xF) as u32, 16).unwrap());
    }
    s
}

fn word(tail: &[u8]) -> String {
    let mut w = [0u8; 32];
    w[32 - tail.len()..].copy_from_slice(tail);
    hex_of(&w)
}

/// A receipt with `status`, paying `amount` of the token from BUYER to the payee.
pub fn receipt(status: &str, amount: u128) -> String {
    let tt = hex_of(&TRANSFER_TOPIC);
    let paid = to();
    let (from, dst, data) = (word(&BUYER), word(&paid.payee), word(&amount.to_be_bytes()));
    let nox = hex_of(&paid.token);
    format!(
        "{{\"result\":{{\"status\":\"{status}\",\"logs\":[{{\"address\":\"{nox}\",\"topics\":[\"{tt}\",\"{from}\",\"{dst}\"],\"data\":\"{data}\"}}]}}}}"
    )
}

pub fn paying(amount: u128) -> String {
    receipt("0x1", amount)
}

pub fn price() -> u128 {
    price_of(ToolId::Recon.id()).unwrap()
}
