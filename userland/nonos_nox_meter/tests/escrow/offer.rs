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

//! ORDER_OFFER's job (plan/29): FETCH 1 first, then each PAGE of the sealed
//! answer by buyer_pk, then the offer opened, checked and kept; a page out of
//! order, a wrong total, another release's answer or an offer that does not
//! make chunk_root ends the job.

use nonos_nox_meter::flow::{Flow, Next, Work};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::OfferMade;
use nonos_settle_proto::Page;

use super::host::{keccak, Host};
use super::offer_run::{run, work};
use super::seller::{root, sealed};
use super::x::D;

#[test]
fn an_offer_is_fetched_page_by_page_opened_checked_and_kept() {
    let w = work(root());
    let answer = sealed(&w.buyer_pk, &[1; 32], &[2; 32]);
    assert!(answer.len() > 60 * 1024, "more than one page");
    let Next::KeepOffer(kept, made) = run(w.clone(), &answer).unwrap() else { panic!("not kept") };
    let m = OfferMade::parse(&made).unwrap();
    assert_eq!(m.offer_id, keccak(&[m.cipher_root, m.key_commit].concat()));
    assert_eq!(
        (kept.offer.cipher_root, kept.buyer_sk, kept.made_at),
        (m.cipher_root, w.buyer_sk, 9)
    );
    assert_eq!((kept.listing_key, kept.chunk_root, kept.content_len), ([1; 32], root(), 70_000));
}

#[test]
fn anything_else_ends_the_job() {
    let w = work(root());
    let other = sealed(&w.buyer_pk, &[1; 32], &[3; 32]);
    assert_eq!(run(w.clone(), &other), Err(MeterError::Untrusted), "another release's answer");
    let answer = sealed(&w.buyer_pk, &[1; 32], &[2; 32]);
    assert_eq!(
        run(work([9; 32]), &answer),
        Err(MeterError::Untrusted),
        "not the release's chunk_root"
    );
    assert_eq!(
        run(w.clone(), &answer[..answer.len() - 1]),
        Err(MeterError::Mismatch),
        "a total one short"
    );
    let (mut f, _) = Flow::begin(Work::Offer(Box::new(w)));
    let late = Page::of(&answer, 61_440).unwrap();
    assert_eq!(f.on_page(&late, (&Host, &D)), Err(MeterError::Mismatch), "a page out of order");
}
