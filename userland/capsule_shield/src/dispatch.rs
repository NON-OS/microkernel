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

//! Each op to its answer: the quick ones at once, the long ones on the
//! worker. Only a wallet window's request reaches here (service.rs).

use shield_wire::*;

use crate::answers::{answer, borrowed, job, now, owned};
use crate::secret::opening;
use crate::{jobs, ops};

pub(crate) fn dispatch(op: u16, body: &str) -> (i32, String) {
    let fields: Vec<&str> = if body.is_empty() { Vec::new() } else { body.split('\n').collect() };
    let f = owned(&fields);
    match op {
        OP_STATE => now(ops::state()),
        OP_OPEN_WORDS => job("open", opening(f, ops::open_words)),
        OP_OPEN_KEY => job("open", opening(f, ops::open_key)),
        OP_LOCK => now(ops::lock()),
        OP_RESULT => answer(STATUS_OK, &jobs::result()),
        OP_CANCEL => now(ops::cancel()),
        OP_FRESH_ADDRESS if fields.first() == Some(&FRESH_ONCE) => {
            now(crate::fresh_once::fresh_once())
        }
        OP_FRESH_ADDRESS if fields.first() == Some(&FRESH_HANDED) => {
            now(crate::fresh_once::fresh_handed())
        }
        OP_FRESH_ADDRESS if fields.first() == Some(&FRESH_BATCH) => {
            job("scan", move || crate::fresh_once::fresh_batch(&borrowed(&f)))
        }
        OP_FRESH_ADDRESS => now(ops::fresh_address()),
        OP_SYNC => job("sync", ops::sync),
        OP_REVIEW_SHIELD => job("review shield", move || ops::review_shield(&borrowed(&f))),
        OP_CONFIRM => job("confirm", move || ops::confirm(&borrowed(&f))),
        OP_QUOTE => job("quote", move || ops::quote(&borrowed(&f))),
        OP_SEND => job("send", move || ops::send(&borrowed(&f))),
        OP_WITHDRAW => job("withdraw", move || ops::withdraw(&borrowed(&f))),
        OP_FOLLOW => job("follow", ops::follow),
        OP_REVIEW_SELF_SETTLE => {
            job("review settle", move || ops::review_self_settle(&borrowed(&f)))
        }
        OP_TAKE_BACK => job("take back", ops::take_back),
        OP_SIGN_FROM_FRESH => job("sign", move || crate::fresh::sign_from_fresh(&borrowed(&f))),
        _ => (STATUS_MALFORMED, String::new()),
    }
}
