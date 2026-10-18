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
//! The payee line on a paid listing's detail (B9, the the consent
//! ruling): the commerce block's payee, from the same OP_GET_COMMERCE
//! record the wallet's consent reads its "pay to" from, in full and in its
//! EIP-55 spelling, so a person can check one screen against the other.
//! Said only where a price is.

use alloc::vec::Vec;

use nonos_marketplace_abi::{address_text, Keccak};

use super::price::{Price, FREE};

pub const CHECKED: &[u8] = b" (checked by the wallet before you buy)";

pub fn payee_line(k: &impl Keccak, p: &Price) -> Option<Vec<u8>> {
    if p.mode == FREE || !p.trusted {
        return None;
    }
    Some([&b"pays: "[..], &address_text(k, &p.payee), CHECKED].concat())
}
