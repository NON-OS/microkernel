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

//! A kept offer (plan/29) reads back exactly, and lives OFFER_TTL.

use nonos_nox_meter::escrow::KeptOffer;
use nonos_nox_meter::flow::Next;

use super::offer_run::{run, work};
use super::seller::{root, sealed};

#[test]
fn a_kept_offer_reads_back_and_ages_out() {
    let w = work(root());
    let Ok(Next::KeepOffer(kept, _)) = run(w.clone(), &sealed(&w.buyer_pk, &[1; 32], &[2; 32]))
    else {
        panic!()
    };
    let b = kept.encode();
    assert_eq!(KeptOffer::decode(&b).as_ref(), Some(&*kept));
    assert_eq!(KeptOffer::decode(&b[..b.len() - 1]), None, "not its content_len's");
    assert!(
        !kept.expired(9 + 86_399) && kept.expired(9 + 86_400) && kept.expired(8),
        "OFFER_TTL, and never from the future"
    );
}
