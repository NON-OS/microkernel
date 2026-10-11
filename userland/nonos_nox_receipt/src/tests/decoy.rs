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

use super::fixtures::*;
use crate::verify::verify_payment;

#[test]
fn a_decoy_transfer_does_not_hide_the_real_one() {
    let tt = transfer_topic();
    let buyer = addr_topic(&BUYER);
    let decoy_to = addr_topic(&[0xDEu8; 20]);
    let payee_to = addr_topic(&PAYEE);
    let amt = amount_word(PRICE);
    let nox = token_addr();
    let r = format!(
        "{{\"result\":{{\"status\":\"0x1\",\"logs\":[\
         {{\"address\":\"{nox}\",\"topics\":[\"{tt}\",\"{buyer}\",\"{decoy_to}\"],\"data\":\"{amt}\"}},\
         {{\"address\":\"{nox}\",\"topics\":[\"{tt}\",\"{buyer}\",\"{payee_to}\"],\"data\":\"{amt}\"}}\
         ]}}}}"
    );
    let p = verify_payment(r.as_bytes(), &PAYEE, PRICE, &token()).expect("second log pays");
    assert_eq!(p.from, BUYER);
}
