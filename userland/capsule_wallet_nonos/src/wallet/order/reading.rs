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
 * v1.4.36): GET_COMMERCE, the release, INSTALL_READY on the names' chain,
 * then GET_APP, so the window never waits on more than one reply. Which
 * call comes next is reading_step.rs; what its answer makes,
 * terms/answer.rs, with keccak from the crypto service.
 */

use nonos_payment_proto::Names;

use super::market::ask;
use super::reading_step::call;
pub use super::reading_step::{Read, Reading};
use super::terms::answer::answered;
use crate::wallet::publish::Kec;

pub fn advance(names: &Names, at: Reading) -> Read {
    match one(names, at) {
        Ok(r) => r,
        Err(why) => Read::Done(Err(why)),
    }
}

fn one(names: &Names, at: Reading) -> Result<Read, &'static str> {
    let (op, body) = call(names, &at)?;
    answered(&Kec, names, at, &ask(op, &body)?)
}
