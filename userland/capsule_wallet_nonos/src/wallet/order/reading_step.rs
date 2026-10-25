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
 * the release, INSTALL_READY on the names' chain, then the listing itself
 * (GET_APP), each answer one call on. read_named judges the terms after
 * INSTALL_READY; GET_APP's answer ends it, once every answer is shown to
 * be for the names asked (terms/asked.rs). What each answer makes is
 * terms/answer.rs.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_market_proto::{listing_body, pair_body, ready_body, Commerce, ReleaseEscrow};
use nonos_market_proto::{OP_GET_APP, OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY};
use nonos_payment_proto::Names;

use super::read::Named;
use super::terms::asked::Record;

pub const TOO_LONG: &str = "That request is too long to send.";
pub const UNREAD: &str = "The market's answer could not be read, so nothing can be confirmed.";

/// Where the reading stands: the next call to make, with what came back.
pub enum Reading {
    Commerce,
    Release(Box<Commerce>),
    /// The release's escrow fields, and its id text as the market resolved it.
    Ready(Box<Commerce>, Box<ReleaseEscrow>, Vec<u8>),
    /// The terms as judged, waiting on the listing's own answer.
    App(Box<Commerce>, Box<Named>, Vec<u8>),
}

/// The terms, and the market's own names for what they buy.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Terms {
    pub named: Named,
    pub record: Record,
}

pub enum Read {
    More(Reading),
    Done(Result<Box<Terms>, &'static str>),
}

/// The market call `at` stands for: its op, and its body for `names`.
pub fn call(names: &Names, at: &Reading) -> Result<(u16, Vec<u8>), &'static str> {
    let (l, r) = (&names.listing[..], &names.release[..]);
    Ok(match at {
        Reading::Commerce => (OP_GET_COMMERCE, pair_body(l, r).ok_or(TOO_LONG)?),
        Reading::Release(_) => (OP_GET_RELEASE, pair_body(l, r).ok_or(TOO_LONG)?),
        Reading::Ready(..) => (OP_INSTALL_READY, ready_body(l, r, names.chain_id).ok_or(TOO_LONG)?),
        Reading::App(..) => (OP_GET_APP, listing_body(l).ok_or(TOO_LONG)?),
    })
}
