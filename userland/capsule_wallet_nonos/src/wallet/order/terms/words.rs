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

//! The escrow consent's sentences, and the payee written in full.

use alloc::format;
use alloc::string::String;

use nonos_marketplace_abi::Keccak;
use nonos_payment_proto::Names;

use crate::wallet::event::address_text::checksummed;
use crate::wallet::order::buy::{Buy, DELIVERY_ASKED};

const DAY: u64 = 86_400;
const _: () = assert!(DELIVERY_ASKED.is_multiple_of(DAY), "said in whole days");
/// The days the consent says: order_new's own deadline, from the order.
pub const DELIVERY_DAYS: u64 = DELIVERY_ASKED / DAY;

pub const QUOTING: &str = "Asking the Shield what funding this price costs. It decides whether \
     the price is one of its standard amounts today.";
pub const REFUSED: &str = "This price cannot be paid privately yet, so the wallet will not pay it.";

/// The escrow's terms, with the deadline the order will carry.
pub fn escrow() -> String {
    format!(
        "Held in escrow until delivery; if it is not delivered within {DELIVERY_DAYS} days it \
         comes back to a fresh address of this wallet."
    )
}

/// Bond and package, then what the store called it.
pub fn detail(n: &Names, b: &Buy) -> String {
    let bond: String = b.bond_id.iter().map(|x| format!("{x:02x}")).collect();
    let release = match n.release.is_empty() {
        true => String::from("its default release"),
        false => text(&n.release),
    };
    format!(
        "Detail: bond 0x{bond}, checked by the market; package {} bytes, checked chunk by \
         chunk; the store called it {}, {release}.",
        b.content_len,
        text(&n.listing)
    )
}

/// `a` in full, 0x and its EIP-55 spelling.
pub fn payee_text(k: &impl Keccak, a: &[u8; 20]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut hex = [0u8; 40];
    for (i, b) in a.iter().enumerate() {
        (hex[2 * i], hex[2 * i + 1]) = (HEX[usize::from(b >> 4)], HEX[usize::from(b & 15)]);
    }
    let spelt = checksummed(&hex, &k.keccak256(&hex));
    format!("0x{}", String::from_utf8_lossy(&spelt))
}

pub fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}
