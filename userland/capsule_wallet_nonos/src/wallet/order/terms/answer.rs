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
 * What each market answer makes of a named purchase's reading (B8): the
 * next call, or once GET_APP answered, the terms with the market's names
 * for them. An answer that does not read, terms read_named refuses, or an
 * answer for other names than the store's ends it there.
 */

use alloc::boxed::Box;

use nonos_market_proto::parse_release_escrow;
use nonos_market_proto::{parse_app_ids, parse_commerce, parse_readiness, parse_release};
use nonos_marketplace_abi::Keccak;
use nonos_payment_proto::Names;

use super::asked::as_asked;
use crate::wallet::order::read::read_named;
use crate::wallet::order::reading_step::{Read, Reading, Terms, UNREAD};

/// What `body`, the answer to `at`'s call for `names`, makes of the reading.
pub fn answered(
    k: &impl Keccak,
    names: &Names,
    at: Reading,
    body: &[u8],
) -> Result<Read, &'static str> {
    Ok(match at {
        Reading::Commerce => {
            let c = parse_commerce(body).ok_or(UNREAD)?;
            Read::More(Reading::Release(Box::new(c)))
        }
        Reading::Release(c) => {
            let e = parse_release_escrow(body).ok_or(UNREAD)?;
            let version = parse_release(body).ok_or(UNREAD)?.version;
            Read::More(Reading::Ready(c, Box::new(e), version))
        }
        Reading::Ready(c, e, version) => {
            let ready = parse_readiness(body).ok_or(UNREAD)?;
            match read_named(names.chain_id, &c, &e, &ready) {
                Ok(named) => Read::More(Reading::App(c, Box::new(named), version)),
                Err(why) => Read::Done(Err(why)),
            }
        }
        Reading::App(c, named, version) => {
            let app = parse_app_ids(body).ok_or(UNREAD)?;
            let record = as_asked(k, names, &app, &c, &version);
            Read::Done(record.map(|record| Box::new(Terms { named: *named, record })))
        }
    })
}
