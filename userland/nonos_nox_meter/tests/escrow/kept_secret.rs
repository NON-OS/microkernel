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

//! A kept offer's buyer key is never printed (the owner's rule 5).

use super::deliver_fixture::kept;

#[test]
fn debug_names_every_field_but_the_buyer_key() {
    let mut k = kept();
    k.buyer_sk = core::array::from_fn(|i| 200 + i as u8 % 50);
    let printed = format!("{k:?}");
    assert!(printed.contains("buyer_sk: \"(redacted)\""));
    assert!(printed.contains("chain_id") && printed.contains("chunk_root"));
    assert!(!printed.contains(&format!("{:?}", k.buyer_sk)), "the key's bytes");
    assert!(!printed.contains("200, 201, 202"), "nor a run of them");
}
