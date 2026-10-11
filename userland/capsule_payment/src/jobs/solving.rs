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

//! A walk under way: its job id, the walk and its solver, and what the
//! offer job needs besides the walk's answer.

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_equix::Solver;
use nonos_nox_meter::escrow::{Asked, OfferWork, Walk};
use nonos_nox_meter::flow::Work as Job;
use nonos_payment_proto::EINVAL;
use nonos_settle_proto::Fetch;

use crate::meter::now_ms;

pub(crate) struct Solving {
    pub id: u32,
    pub walk: Walk,
    pub solver: Box<Solver>,
    pub chunk_root: [u8; 32],
    pub content_len: u64,
}

impl Solving {
    /// The offer job for the walk's answer: FETCH 1 ESCROW, then its pages.
    pub(super) fn job(&self, asked: Asked) -> Result<Job, i32> {
        let buyer_pk = asked.request.buyer_pk;
        let request = Fetch::Escrow(asked.request.clone()).encode().ok_or(EINVAL)?;
        Ok(Job::Offer(Box::new(OfferWork {
            want: self.walk.want().clone(),
            chunk_root: self.chunk_root,
            content_len: self.content_len,
            buyer_sk: asked.buyer_sk,
            buyer_pk,
            request,
            made_at: now_ms() / 1000,
            fetched: Vec::new(),
        })))
    }
}
