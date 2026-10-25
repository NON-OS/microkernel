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

//! Where the capsule's HMAC and HKDF wipe, read from the source: every
//! buffer that holds the key or a block, on either outcome.

use super::wipe_glue_tests::before;

const HMAC: &str = include_str!("../../capsule_net_nym/src/crypto/hkdf_blake3/hmac.rs");
const EXPAND: &str = include_str!("../../capsule_net_nym/src/crypto/hkdf_blake3/expand.rs");

#[test]
fn hkdf_wipes_every_buffer_that_holds_the_key_or_a_block() {
    for w in [
        "wipe(&mut digest);",
        "wipe(&mut inner);",
        "wipe(&mut padded);",
        "wipe(&mut inner_digest);",
        "wipe(&mut outer);",
    ] {
        assert!(HMAC.contains(w), "hmac: {w}");
    }
    assert!(before(EXPAND, "wipe(&mut message);", "if result.is_err() {"));
    assert!(before(EXPAND, "wipe(&mut previous);", "    result\n}"));
}
