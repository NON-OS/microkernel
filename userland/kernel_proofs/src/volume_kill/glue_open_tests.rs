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

//! The opens under one hold (G12, and the HELD case): nothing the hold
//! covers takes the volume again, and a live boot's open has no round trip
//! after its RAM volume starts, so no opener can be swept between HELD's
//! store and the install.

use crate::proofs::proofs_8::transport_takeover::text::{code, tail};

const UNDER_HOLD: [(&str, &str); 12] = [
    ("open_into", include_str!("../../../../src/fs/blockfs_volume/open_into.rs")),
    ("opened", include_str!("../../../../src/fs/blockfs_volume/opened.rs")),
    ("passphrase_create", include_str!("../../../../src/fs/blockfs_volume/passphrase_create.rs")),
    ("passphrase_open", include_str!("../../../../src/fs/blockfs_volume/passphrase_open.rs")),
    ("passphrase_key", include_str!("../../../../src/fs/blockfs_volume/passphrase_key.rs")),
    ("plan_read", include_str!("../../../../src/fs/blockfs_volume/plan_read.rs")),
    ("plan", include_str!("../../../../src/fs/blockfs_volume/plan.rs")),
    ("key_header_io", include_str!("../../../../src/fs/blockfs_volume/key_header_io.rs")),
    ("key_header", include_str!("../../../../src/fs/blockfs_volume/key_header.rs")),
    ("mount_or_format", include_str!("../../../../src/fs/blockfs_volume/mount_or_format.rs")),
    ("ring_blank", include_str!("../../../../src/fs/blockfs_volume/ring_blank.rs")),
    ("key_seal", include_str!("../../../../src/fs/blockfs_volume/key_seal.rs")),
];
const SESSION: &str = include_str!("../../../../src/fs/blockfs_volume/session.rs");

#[test]
fn g12_nothing_under_an_opens_hold_takes_the_volume_again() {
    for (name, src) in UNDER_HOLD {
        let c = code(src);
        for again in ["VOLUME.", "blockfs_volume::", "keeping_on(", "open_session_volume("] {
            assert!(!c.contains(again), "{name} calls {again} under the open's hold");
        }
    }
}

#[test]
fn held_is_stored_after_the_install_with_no_round_trip_between() {
    let open = tail(SESSION, "pub(super) fn open_session_volume(");
    let at = |n: &str| open.find(&code(n)).unwrap_or_else(|| panic!("`{n}` in the open"));
    let order = [
        at("let mut volume = VOLUME.write();"),
        at("fill_random_bytes_secure(&mut key)"),
        at("fill_random_bytes_secure(&mut uuid)"),
        at("ram::start(sectors)"),
        at("*volume = Some(VolumeState { key, mount });"),
        at("keep_switch::HELD.store(held,"),
    ];
    assert!(order.windows(2).all(|w| w[0] < w[1]), "{order:?}");
    let after = &open[at("ram::start(sectors)")..];
    let trips =
        ["fill_random", "retrieve(", "store(KeyType", "derive_for_kernel", "block_device::"];
    for trip in trips {
        assert!(!after.contains(trip), "a round trip, `{trip}`, after the RAM volume starts");
    }
}
