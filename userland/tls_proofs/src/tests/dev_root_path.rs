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

//! The throwaway root's path length is zero: it signs the leaf itself or
//! nothing. An intermediate it signed, and a chain that repeats the root,
//! are refused by every build, the default lane's and the devnet lane's.

use super::dev_root::{HOST, LEAF, ROOT};
use super::dev_root_refusals::{refused, sound};

static INTER: &[u8] = include_bytes!("../../../../tools/devnet/tls/inter.der");
static INTER_LEAF: &[u8] = include_bytes!("../../../../tools/devnet/tls/inter-leaf.der");

#[test]
fn an_intermediate_under_the_root_anchors_nothing() {
    sound(INTER_LEAF, INTER, HOST);
    let root = crate::cert_spki::cert_spki(ROOT).expect("spki");
    assert_eq!(crate::verify_link::verify_link(INTER, root), Ok(true));
    assert!(crate::cert_is_ca::cert_is_ca(INTER));
    refused(&[INTER_LEAF, INTER], HOST);
    refused(&[INTER_LEAF, INTER, ROOT], HOST);
}
#[test]
fn the_local_rpc_leaf_with_the_root_twice_is_refused() {
    refused(&[LEAF, ROOT, ROOT], HOST);
}
