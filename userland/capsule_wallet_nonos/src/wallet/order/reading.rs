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
 * A named purchase's terms read from market.index one call a tick (B8,
 * v1.4.36): GET_COMMERCE, the release's escrow fields, then INSTALL_READY
 * on the names' chain, so the window never waits on more than one reply.
 */

use alloc::boxed::Box;

use nonos_market_proto::{pair_body, parse_commerce, parse_readiness, parse_release_escrow};
use nonos_market_proto::{ready_body, Commerce, ReleaseEscrow};
use nonos_market_proto::{OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY};
use nonos_payment_proto::Names;

use super::market::{ask, TOO_LONG, UNREAD};
use super::read::{read_named, Named};

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

pub fn advance(names: &Names, at: Reading) -> Read {
    match one(names, at) {
        Ok(r) => r,
        Err(why) => Read::Done(Err(why)),
    }
}

fn one(names: &Names, at: Reading) -> Result<Read, &'static str> {
    let (l, r) = (&names.listing[..], &names.release[..]);
    Ok(match at {
        Reading::Commerce => {
            let c = parse_commerce(&ask(OP_GET_COMMERCE, &pair_body(l, r).ok_or(TOO_LONG)?)?)
                .ok_or(UNREAD)?;
            Read::More(Reading::Release(Box::new(c)))
        }
        Reading::Release(c) => {
            let body = ask(OP_GET_RELEASE, &pair_body(l, r).ok_or(TOO_LONG)?)?;
            let e = parse_release_escrow(&body).ok_or(UNREAD)?;
            Read::More(Reading::Ready(c, Box::new(e)))
        }
        Reading::Ready(c, e) => {
            let body = ask(OP_INSTALL_READY, &ready_body(l, r, names.chain_id).ok_or(TOO_LONG)?)?;
            let ready = parse_readiness(&body).ok_or(UNREAD)?;
            Read::Done(read_named(names.chain_id, &c, &e, &ready))
        }
    })
}
