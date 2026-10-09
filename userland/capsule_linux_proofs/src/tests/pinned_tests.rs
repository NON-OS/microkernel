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

use super::pinned_published::{kept_as, KEPT_AS, PUBLISHED, TIERS};
use crate::model_name::volume_name;
use crate::models::pinned::{all, keepable, pin_of, ENTRY_BYTES, STREAM_NAME_MAX};

/*
 * The volume's name field and the longest name the kernel streams, in
 * bytes, written out here from src/fs/blockfs/dir_consts.rs and
 * src/fs/blockfs_volume/import_feed/live.rs; and its largest file.
 */
const NAME_BYTES: usize = 56;
const KERNEL_STREAM_NAME_MAX: usize = 49;
const MAX_FILE_BYTES: u64 = 382_737_381_576;

#[test]
fn every_pin_is_the_published_file_byte_for_byte() {
    assert_eq!(all().count(), PUBLISHED.len());
    for (p, (tier, name, bytes, hex)) in all().zip(PUBLISHED) {
        assert_eq!(p.tier, tier, "{name}");
        assert_eq!(&p.name[1..], kept_as(name).as_bytes());
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

/*
 * The published names kept under shorter ones could not be kept under
 * their own, and a short name stays a part of the same model: the same
 * stem for every part, the part's "-0000N-of-0000M.gguf" unchanged, which
 * is how the chat program finds the other parts from the first.
 */
#[test]
fn renamed_files_needed_it_and_stay_one_split_model() {
    for (published, kept) in KEPT_AS {
        let long = alloc::format!("/{published}");
        assert!(!keepable(long.as_bytes()), "{published} did not need a new name");
        let tail = &published[published.len() - "-00001-of-00002.gguf".len()..];
        assert!(tail.starts_with("-0000") && tail.contains("-of-0000"), "{published}");
        assert!(kept.ends_with(tail), "{kept}");
        assert!(PUBLISHED.iter().any(|(_, n, _, _)| *n == published), "{published}");
    }
    for (published, kept) in KEPT_AS {
        let stem = &kept[..kept.len() - "-00001-of-00002.gguf".len()];
        let tier = PUBLISHED.iter().find(|(_, n, _, _)| *n == published).unwrap().0;
        for (other, k) in KEPT_AS {
            let other_tier = PUBLISHED.iter().find(|(_, n, _, _)| *n == other).unwrap().0;
            assert_eq!(k.starts_with(stem), tier == other_tier, "{kept} and {k}");
        }
    }
}

/* The small tiers ek boots most, by name, as the volume keeps them. */
#[test]
fn the_smallest_and_the_4b_tiers_keep_their_published_names() {
    for (tier, name) in [
        ("small", "/qwen2.5-0.5b-instruct-q4_k_m.gguf"),
        ("qwen3-0.6b", "/Qwen3-0.6B-Q8_0.gguf"),
        ("qwen3-4b", "/Qwen3-4B-Q4_K_M.gguf"),
    ] {
        let p = pin_of(name.as_bytes()).unwrap();
        assert_eq!(p.tier, tier);
        assert!(keepable(p.name));
        assert_eq!(kept_as(&name[1..]), &name[1..]);
    }
}
