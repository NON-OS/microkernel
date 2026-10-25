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
 * The last checks at Sign, once the chain's pending nonce is read: the
 * network still the send's, the calldata still one encode writes, the
 * quote still the one reviewed and its nonce still the chain's (Q8), and
 * the quote still held to the intent. Only then the fields to sign, whose
 * data is the send's own calldata (D5).
 */

use nonos_settle_proto::Quote;

use super::call::Publisher;
use super::decode::decode;
use super::review_bond::At;
use super::send::{signable, PublisherSend, Signable};
use super::step::{sign, IntentStep};

pub const CHAIN_MOVED: &str = "the network changed since this review; review it again";
pub const NOT_READ_BACK: &str = "the wallet could not read back what it would sign; not signing";

/// `chain_id` is the network now; `held` the quote's bounds for the call.
pub fn ready(
    send: &PublisherSend,
    step: &IntentStep,
    chain_id: u64,
    pending_nonce: u64,
    at: At,
    held: impl Fn(&Publisher, &Quote) -> Option<&'static str>,
) -> Result<Signable, &'static str> {
    if chain_id != send.chain_id {
        return Err(CHAIN_MOVED);
    }
    let p = decode(send.kind, &send.calldata, at).ok_or(NOT_READ_BACK)?;
    let q = sign(step, pending_nonce)?;
    match held(&p, &q) {
        Some(why) => Err(why),
        None => Ok(signable(send, &q)),
    }
}
