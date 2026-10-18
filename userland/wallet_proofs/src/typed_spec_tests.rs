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

//! The domain separator against the EIP-712 specification's own example.

use super::typed_hash_tests::{hex, keccak};
use crate::server::typed::domain::Domain;
use crate::server::typed::preimage::domain_preimage;

/// EIP-712's "Ether Mail" domain: name "Ether Mail", version "1", chain 1,
/// contract 0xCcCC..cccC, whose separator the specification gives.
#[test]
fn the_domain_separator_matches_the_specification() {
    let mail = Domain { name: "Ether Mail", version: "1" };
    let pre = domain_preimage(&mail, 1, &[0xCC; 20], keccak).unwrap();
    assert_eq!(
        hex(&keccak(&pre).unwrap()),
        "f2cee375fa42b42143804025fc449deafd50cc031ca257e0b194a650a912090f"
    );
}
