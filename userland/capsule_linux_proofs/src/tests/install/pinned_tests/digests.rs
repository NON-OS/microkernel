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

//! Each pin the published file, renamed files kept whole, published names kept.

use super::super::pinned_published::{kept_as, KEPT_AS, PUBLISHED};
use crate::models::pinned::{all, keep::keepable, pin_of};

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
