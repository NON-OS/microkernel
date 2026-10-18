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

//! A family still serving, as MkSealRead's live form hands it over, and a
//! DELIVER asking for it (plan/15).

use nonos_proof_abi::wire::DeliverRequest;
use nonos_proof_abi::Record;

use super::{hist, policy, record, sealed_bytes, LISTING};
use crate::market::sealed::{split, Sealed};

pub fn live() -> (Sealed, [u8; 256]) {
    let p = policy(&LISTING, &[0, 1, 60, 231]);
    let rec = Record { flags: 0, ..record(&p, &hist(), &[[0x11; 32]]) };
    (split(&sealed_bytes(&rec, &hist(), &[[0x11; 32]])).expect("splits"), p)
}

pub fn ask<'a>(p: &'a [u8; 256], request: &'a [u8], response: &'a [u8]) -> DeliverRequest<'a> {
    let (n, o, c) = (&[0x6E; 32], &[0x0D; 32], &[0xC4; 32]);
    DeliverRequest {
        listing_id: &LISTING,
        policy: p,
        nonce: n,
        order_id: o,
        channel_id: c,
        from: 1,
        to: 2,
        request,
        response,
    }
}
