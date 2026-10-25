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
//! admit() on the voucher where its contract is deployed on one chain and
//! not the other.

use nonos_nox_config::{MAINNET, SEPOLIA};

use super::typed_fixture::sepolia_only;
use crate::server::typed::check::admit_in;
use crate::server::typed::refusal::Refusal;
use crate::server::typed::voucher::VOUCHER;

#[test]
fn admits_a_deployed_chain_and_refuses_the_other() {
    let words = vec![0u8; 9 * 32];
    assert_eq!(admit_in(&VOUCHER, SEPOLIA, &words, sepolia_only), Ok([7; 20]));
    assert_eq!(admit_in(&VOUCHER, MAINNET, &words, sepolia_only), Err(Refusal::NotDeployed));
    let short = &words[..32];
    assert_eq!(admit_in(&VOUCHER, SEPOLIA, short, sepolia_only), Err(Refusal::Malformed));
    let mut wide = words.clone();
    wide[32] = 1;
    assert_eq!(admit_in(&VOUCHER, SEPOLIA, &wide, sepolia_only), Err(Refusal::Field(1)));
    assert_eq!(admit_in(&VOUCHER, 5, &words, sepolia_only), Err(Refusal::Chain));
}
