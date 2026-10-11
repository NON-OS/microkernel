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

use nonos_marketplace_abi::Commerce;
use nonos_nox_meter::pay::{priced, Offer};
use nonos_nox_meter::MeterError;

use super::fixture::{commerce, offer, request, NOX};

#[test]
fn a_request_equal_to_the_block_is_priced_qty_times_unit_price() {
    assert_eq!(priced(&request(), &offer(&commerce()), &NOX), Ok(35));
}

#[test]
fn any_word_off_the_block_or_an_untrusted_release_is_refused() {
    let r = request();
    let good = offer(&commerce());
    assert_eq!(
        priced(&r, &Offer { trusted: false, ..good.clone() }, &NOX),
        Err(MeterError::Untrusted)
    );
    for bad in [
        Offer { listing_id: [0; 32], ..good.clone() },
        Offer { release_hash: [0; 32], ..good.clone() },
        Offer { chain_id: 11_155_111, ..good.clone() },
        Offer { token: vec![0; 20], ..good.clone() },
        offer(&Commerce { payee: [0xcc; 20], ..commerce() }),
        offer(&Commerce { unit: [8; 32], ..commerce() }),
        offer(&Commerce { unit_price: 8, ..commerce() }),
        Offer { block: vec![1, 2, 3], ..good.clone() },
    ] {
        assert_eq!(priced(&r, &bad, &NOX), Err(MeterError::Mismatch), "{bad:?}");
    }
}
