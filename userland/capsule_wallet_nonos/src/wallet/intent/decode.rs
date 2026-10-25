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
 * A publisher call read back from its calldata, so the review shows what
 * is signed. The kind is passed in: 17, 18 and 19 share withdrawCredit,
 * only 19 is the publisher's. What encode would not write is refused.
 */

use nonos_nox_config::Contract;
use nonos_settle_proto::https_target;

use super::call::Publisher;
use super::decode_words::{address, amount, exact, heads, string, uint8, SPENDERS};
use super::encode::selector;

pub fn decode(
    kind: u8,
    data: &[u8],
    at: impl Fn(Contract) -> Option<[u8; 20]>,
) -> Option<Publisher> {
    use Publisher::*;
    let sel = selector(kind)?;
    Some(match kind {
        1 => {
            let [s, v] = exact(data, sel)?;
            Approve { spender: spender(address(&s)?, at)?, amount: amount(&v)? }
        }
        2 => {
            let [publisher_key, v] = exact(data, sel)?;
            Bond { publisher_key, amount: amount(&v)? }
        }
        3 => {
            let [bond_id, a] = exact(data, sel)?;
            SetDeliveryAgent { bond_id, agent: address(&a)? }
        }
        4 => RequestUnbond { bond_id: exact::<1>(data, sel)?[0] },
        5 => WithdrawBond { bond_id: exact::<1>(data, sel)?[0] },
        6 => {
            let [publisher_key, app_id_hash, release_id, release_hash, bond_id] = heads(data, sel)?;
            let uri = string(data, 5).filter(|u| https_target(u).is_some())?;
            Publish { publisher_key, app_id_hash, release_id, release_hash, bond_id, uri }
        }
        7 => {
            let [listing_id, release_id] = exact(data, sel)?;
            WithdrawRelease { listing_id, release_id }
        }
        8 => {
            let [anchor, k, listing_id] = exact(data, sel)?;
            Anchor { anchor, kind: uint8(&k)?, listing_id }
        }
        19 => WithdrawBondCredit { to: address(&exact::<1>(data, sel)?[0])? },
        _ => return None,
    })
}

/// The one spender `at` names `a` for; none, or two, is refused.
fn spender(a: [u8; 20], at: impl Fn(Contract) -> Option<[u8; 20]>) -> Option<Contract> {
    let mut named = SPENDERS.into_iter().filter(|c| at(*c) == Some(a));
    let one = named.next()?;
    named.next().is_none().then_some(one)
}
