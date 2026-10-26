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

//! Each pinned name one a guest can open, and every pin within the volume.

use super::super::pinned_published::TIERS;
use crate::model_name::volume_name;
use crate::models::pinned::keep::{keepable, ENTRY_BYTES, STREAM_NAME_MAX};
use crate::models::pinned::{all, pin_of};

/*
 * The volume's name field and the longest name the kernel streams, in
 * bytes, written out here from src/fs/blockfs/dir_consts.rs and
 * src/fs/blockfs_volume/import_feed/live.rs; and its largest file.
 */
const NAME_BYTES: usize = 56;
const KERNEL_STREAM_NAME_MAX: usize = 49;
const MAX_FILE_BYTES: u64 = 382_737_381_576;

#[test]
fn every_pinned_name_is_one_a_guest_can_open_and_every_tier_is_there() {
    for (i, p) in all().enumerate() {
        let path = [&b"/models"[..], p.name].concat();
        assert_eq!(volume_name(&path), Some(p.name));
        assert!(keepable(p.name), "{:?}", core::str::from_utf8(p.name));
        assert!(p.bytes > 0 && p.bytes <= MAX_FILE_BYTES);
        assert!(all().skip(i + 1).all(|q| q.name != p.name), "pinned twice");
        assert!(core::ptr::eq(pin_of(p.name).unwrap(), p));
    }
    for tier in TIERS {
        assert!(all().any(|p| p.tier == tier), "{tier}");
    }
    assert!(all().all(|p| TIERS.contains(&p.tier)), "a tier outside the list");
}

#[test]
fn the_limits_are_the_kernels() {
    assert_eq!(ENTRY_BYTES, NAME_BYTES);
    assert_eq!(STREAM_NAME_MAX, KERNEL_STREAM_NAME_MAX);
}

/* Every pinned file, its record and its mark fit, counted out by hand. */
#[test]
fn every_pin_its_record_and_its_mark_fit_the_volume() {
    for p in all() {
        let file = &p.name[1..];
        let name = core::str::from_utf8(p.name).unwrap();
        assert!(p.name.len() <= KERNEL_STREAM_NAME_MAX, "{name}: {} bytes", p.name.len());
        assert!(file.len() + b".sha256".len() <= NAME_BYTES, "{name}.sha256");
        assert!(file.len() + b".partial".len() <= NAME_BYTES, "{name}.partial");
    }
}

#[test]
fn a_name_one_byte_past_the_limit_is_refused() {
    let mut name = alloc::vec![b'/'];
    name.extend(core::iter::repeat_n(b'a', KERNEL_STREAM_NAME_MAX - 1));
    assert!(keepable(&name));
    name.push(b'a');
    assert!(!keepable(&name));
    assert!(!keepable(b"/") && !keepable(b"") && !keepable(b"no-slash.gguf"));
}
