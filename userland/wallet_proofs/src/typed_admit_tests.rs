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

//! admit() on an entry that has a contract on one chain and not the other.

use super::typed_live_tests::LIVE;
use crate::server::typed::check::admit;
use crate::server::typed::refusal::Refusal;

#[test]
fn admits_a_deployed_chain_and_refuses_the_other() {
    let words = [[0u8; 32]; 2].concat();
    assert_eq!(admit(&LIVE, 11_155_111, &words), Ok([0x55; 20]));
    assert_eq!(admit(&LIVE, 1, &words), Err(Refusal::NotDeployed));
    assert_eq!(admit(&LIVE, 11_155_111, &words[..32]), Err(Refusal::Malformed));
    let mut wide = words.clone();
    wide[0] = 1;
    assert_eq!(admit(&LIVE, 11_155_111, &wide), Err(Refusal::Field(0)));
}
