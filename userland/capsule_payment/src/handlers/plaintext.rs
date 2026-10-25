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

//! ORDER_PLAINTEXT (op 18, the plan), from app.linux.install alone: up to a
//! page of an order's checked plaintext from offset. ENOENT before
//! ORDER_DELIVERED's OK; reads repeat, and the plaintext leaves only with the
//! order. The install role checks the escrow pin
//! itself (C5e), recomputing chunk_root and content_len from these bytes.

use alloc::vec::Vec;

use nonos_payment_proto::{PlainAsk, PlainPage, EINVAL};

use crate::orders::plain_of;
use crate::server::Call;
use crate::wipe::wipe;

pub fn order_plaintext(req: &Call<'_>) -> Vec<u8> {
    req.result(page(req.body))
}

fn page(body: &[u8]) -> Result<Vec<u8>, i32> {
    let a = PlainAsk::parse(body).ok_or(EINVAL)?;
    let mut plain = plain_of(&a.order_id)?;
    let page = PlainPage::of(&plain, a.offset).map(|p| p.encode()).ok_or(EINVAL);
    wipe(&mut plain);
    page
}
