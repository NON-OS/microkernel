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

//! An offer job built as the capsule builds it, and run through the flow
//! with each page of an answer as settle pages it.

use nonos_equix::Solver;
use nonos_nox_meter::escrow::{escrow_request, OfferWork, Want};
use nonos_nox_meter::flow::{Ask, Flow, Next, Work};
use nonos_nox_meter::MeterError;
use nonos_settle_proto::{Fetch, Page};

use super::host::Host;
use super::seller::content;
use super::x::D;

pub fn work(chunk_root: [u8; 32]) -> OfferWork {
    let want = Want {
        chain_id: 11_155_111,
        url: b"https://a.example/e".to_vec(),
        listing_key: [1; 32],
        release_key: [2; 32],
    };
    let mut n = 0u8;
    let mut draw = || {
        n += 1;
        Some([n; 32])
    };
    let a = escrow_request(&D, &mut draw, &mut Solver::new().unwrap(), &want).unwrap();
    let request = nonos_settle_proto::Fetch::Escrow(a.request.clone()).encode().unwrap();
    let (buyer_sk, buyer_pk) = (a.buyer_sk, a.request.buyer_pk);
    let content_len = content().len() as u64;
    OfferWork {
        want,
        chunk_root,
        content_len,
        buyer_sk,
        buyer_pk,
        request,
        made_at: 9,
        fetched: Vec::new(),
    }
}

/// Every page of the answer through the flow, settle's way; the last step.
pub fn run(w: OfferWork, answer: &[u8]) -> Result<Next, MeterError> {
    let pk = w.buyer_pk;
    let (mut f, first) = Flow::begin(Work::Offer(Box::new(w.clone())));
    assert_eq!(first, Next::Ask(Ask::Fetch(w.request.clone())), "FETCH 1 first");
    let mut at = 0u32;
    loop {
        let page = Page::of(answer, at).unwrap();
        at += page.bytes.len() as u32;
        match f.on_page(&page, (&Host, &D))? {
            Next::Ask(Ask::Fetch(b)) => {
                assert_eq!(Fetch::parse(&b), Some(Fetch::Page { release_hash: pk, offset: at }))
            }
            last => return Ok(last),
        }
    }
}
