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

//! Open channel's gate and request (B8, v1.4.38): it acts only at consent
//! on terms the market read as a voucher, with no channel kept instead,
//! and the unspent deposit goes back to the fresh address alone.

use nonos_payment_proto::parse_open;

use super::fixture::NOW;
use super::gate_fixture::{helds, names, reads, steps, FRESH};
use super::market_drive::{read_through, voucher_market};
use super::voucher::voucher;
use crate::order::fresh_reply::NO_FRESH;
use crate::order::open_gate::{open_start, openable};
use crate::order::pending_step::Step;
use crate::order::read::Named;

#[test]
fn open_acts_only_at_consent_on_voucher_terms_with_the_form_offered() {
    let mut acted = 0;
    for (i, step) in steps().iter().enumerate() {
        for read in &reads() {
            for held in &helds() {
                let got = openable(step, read, held);
                let voucher_terms = matches!(read, Ok(Named::Voucher(_)));
                if matches!(step, Step::Consent) && voucher_terms && held.offers_form() {
                    assert_eq!(got, Some(&voucher()));
                    acted += 1;
                } else {
                    assert_eq!(got, None, "step {i}, read {read:?}, held {held:?}");
                }
            }
        }
    }
    assert_eq!(acted, 2, "the form offered: no channel, or another asked for");
}

#[test]
fn the_reclaim_address_is_the_fresh_one_never_the_names() {
    let n = names();
    let Ok(Named::Voucher(v)) = read_through(&n, &voucher_market()).1 else { panic!("a voucher") };
    for fresh in [FRESH, [0x01; 20]] {
        let s = open_start(&v, Ok(fresh), v.unit_price * 2, NOW + 60, || NOW).unwrap();
        assert_eq!(
            parse_open(&s.encode()).unwrap().unwrap().reclaim_to,
            fresh,
            "as the meter reads"
        );
        assert!(n.listing.windows(20).chain(n.release.windows(20)).all(|w| w != fresh));
    }
}

#[test]
fn no_fresh_address_or_a_zero_one_opens_nothing() {
    let v = voucher();
    let unread = || -> u64 { panic!("the clock is read only for a fresh address") };
    assert_eq!(open_start(&v, Err(NO_FRESH), v.unit_price, NOW + 1, unread), Err(NO_FRESH));
    assert!(open_start(&v, Ok([0; 20]), v.unit_price, NOW + 1, || NOW).is_err());
    assert!(open_start(&v, Ok(FRESH), v.unit_price, NOW + 1, || NOW).is_ok());
}
