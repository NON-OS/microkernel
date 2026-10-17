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
//! The wallet's "Use it ... install again from the store" (0d2cd87e): the
//! first Enter posts, the wallet opens the channel, the second installs.

use super::fixture::listing;
use super::held_fixture::{row, sold};
use super::meter_fixture::{meter, record};
use crate::store::asked::Asked;
use crate::store::channel_held::{answer, queued, reads_channels, route, Route};
use crate::store::next_step::{on_enter, Enter};
use crate::store::price::{ESCROW, VOUCHER};
use crate::store::progress::Progress;
use crate::store::purchase_said::{handed, ordered};

#[test]
fn install_again_from_the_store_installs_and_does_not_post_twice() {
    let mut l = sold(VOUCHER);
    assert_eq!(on_enter(l.progress, l.removing), Enter::Install);
    assert!(reads_channels(&l));
    assert_eq!(route(&l, answer(Ok(meter(&[], &record(3))))), Route::Post);
    assert!(handed(Ok(())) == Asked::InWallet && !ordered(&l), "posted, and no order to follow");
    // The wallet opened the channel; Enter again.
    assert_eq!(on_enter(l.progress, l.removing), Enter::Install, "posting moved nothing");
    assert_eq!(route(&l, answer(Ok(meter(&[row()], &record(3))))), Route::Install);
    queued(&mut l, Asked::Queued);
    assert_eq!(l.progress, Progress::Queued);
    assert_eq!(on_enter(l.progress, l.removing), Enter::Wait, "nothing asked while it moves");
}

#[test]
fn only_a_queued_install_shows_as_queued() {
    let mut l = sold(VOUCHER);
    (l.progress, l.removing) = (Progress::Removed, true);
    for not in [Asked::Busy, Asked::NotReady, Asked::InImage, Asked::Refused(-22)] {
        queued(&mut l, not);
        assert_eq!((l.progress, l.removing), (Progress::Removed, true));
    }
    queued(&mut l, Asked::Queued);
    assert_eq!((l.progress, l.removing), (Progress::Queued, false));
}

#[test]
fn escrow_still_posts_and_an_unsold_listing_still_installs() {
    assert!(!reads_channels(&sold(ESCROW)) && ordered(&sold(ESCROW)), "no FUNDED asked");
    assert_eq!(route(&sold(ESCROW), Some(true)), Route::Post, "even on a stray yes");
    assert_eq!(route(&listing(b"linux.x", b"X"), None), Route::Install, "no price: as before");
    let mut untrusted = sold(VOUCHER);
    if let Some(p) = untrusted.known.price.as_mut() {
        p.trusted = false;
    }
    assert_eq!(route(&untrusted, None), Route::Install, "by_wallet's answer kept");
}
