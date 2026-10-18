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

//! The hash a policy's net_dests_hash holds.

use super::dests_write::{list, name};
use crate::dests::dests_hash;

#[test]
fn the_hash_is_domain_separated() {
    let l = list(vec![name("example.org", (443, 443))]);
    let mut cat = b"NONOS.linux.netdests.v1".to_vec();
    cat.extend_from_slice(&l);
    assert_eq!(dests_hash(&l), *blake3::hash(&cat).as_bytes());
    assert_ne!(dests_hash(&l), *blake3::hash(&l).as_bytes());
}
