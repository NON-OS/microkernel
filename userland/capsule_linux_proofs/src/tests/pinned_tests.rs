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

//! The model tables: each digest the published one, each name one a guest
//! can open and the volume can hold, and every tier there.

use super::pinned_published::{PUBLISHED, TIERS};
use crate::model_name::volume_name;
use crate::pinned::pinned::{all, pin_of};

/*
 * The volume's name field, in bytes, and its largest file.
 */
const NAME_BYTES: usize = 56;
const MAX_FILE_BYTES: u64 = 382_737_381_576;

#[test]
fn every_pin_is_the_published_file_byte_for_byte() {
    assert_eq!(all().count(), PUBLISHED.len());
    for (p, (tier, name, bytes, hex)) in all().zip(PUBLISHED) {
        assert_eq!(p.tier, tier, "{name}");
        assert_eq!(&p.name[1..], name.as_bytes());
        assert_eq!(p.bytes, bytes, "{name}");
        let got: alloc::string::String =
            p.sha256.iter().map(|b| alloc::format!("{b:02x}")).collect();
        assert_eq!(got, hex, "{name}");
    }
}

#[test]
fn every_pinned_name_is_one_a_guest_can_open_and_every_tier_is_there() {
    for (i, p) in all().enumerate() {
        let path = [&b"/models"[..], p.name].concat();
        assert_eq!(volume_name(&path), Some(p.name));
        assert!(p.name.len() <= NAME_BYTES, "{:?}", p.name);
        assert!(p.bytes > 0 && p.bytes <= MAX_FILE_BYTES);
        assert!(all().skip(i + 1).all(|q| q.name != p.name), "pinned twice");
        assert!(core::ptr::eq(pin_of(p.name).unwrap(), p));
    }
    for tier in TIERS {
        assert!(all().any(|p| p.tier == tier), "{tier}");
    }
    assert!(all().all(|p| TIERS.contains(&p.tier)), "a tier outside the list");
}
