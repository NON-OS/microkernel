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

/*
 * The order a named purchase's terms are read in (B8, v1.4.36): COMMERCE,
 * the release's escrow fields, then INSTALL_READY on the names' chain,
 * each answer one call on; only INSTALL_READY's ends it, in read_named.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_market_proto::{pair_body, parse_commerce, parse_readiness, parse_release_escrow};
use nonos_market_proto::{ready_body, Commerce, ReleaseEscrow};
use nonos_market_proto::{OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY};
use nonos_payment_proto::Names;

use super::read::{read_named, Named};

pub const TOO_LONG: &str = "That request is too long to send.";
pub const UNREAD: &str = "The market's answer could not be read, so nothing can be confirmed.";

/// Where the reading stands: the next call to make, with what came back.
pub enum Reading {
    Commerce,
    Release(Box<Commerce>),
    Ready(Box<Commerce>, Box<ReleaseEscrow>),
}

pub enum Read {
    More(Reading),
    Done(Result<Named, &'static str>),
}

/// The market call `at` stands for: its op, and its body for `names`.
pub fn call(names: &Names, at: &Reading) -> Result<(u16, Vec<u8>), &'static str> {
    let (l, r) = (&names.listing[..], &names.release[..]);
    Ok(match at {
        Reading::Commerce => (OP_GET_COMMERCE, pair_body(l, r).ok_or(TOO_LONG)?),
        Reading::Release(_) => (OP_GET_RELEASE, pair_body(l, r).ok_or(TOO_LONG)?),
        Reading::Ready(..) => (OP_INSTALL_READY, ready_body(l, r, names.chain_id).ok_or(TOO_LONG)?),
    })
}

/// What `body`, the answer to `at`'s call, makes of the reading: the next
/// call, or after INSTALL_READY the purchase as read on `chain_id`.
pub fn answered(chain_id: u64, at: Reading, body: &[u8]) -> Result<Read, &'static str> {
    Ok(match at {
        Reading::Commerce => {
            let c = parse_commerce(body).ok_or(UNREAD)?;
            Read::More(Reading::Release(Box::new(c)))
        }
        Reading::Release(c) => {
            let e = parse_release_escrow(body).ok_or(UNREAD)?;
            Read::More(Reading::Ready(c, Box::new(e)))
        }
        Reading::Ready(c, e) => {
            let ready = parse_readiness(body).ok_or(UNREAD)?;
            Read::Done(read_named(chain_id, &c, &e, &ready))
        }
    })
}
