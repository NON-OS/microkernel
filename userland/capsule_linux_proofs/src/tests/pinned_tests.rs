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

//! The model table: each digest the published one, each name one a guest
//! can open, and every tier there.

use crate::model_name::volume_name;
use crate::pinned::pinned::PINNED;

const PUBLISHED: [(&str, u64, &str); 5] = [
    ("qwen2.5-0.5b-instruct-q4_k_m.gguf", 491_400_032, "74a4da8c9fdbcd15bd1f6d01d621410d31c6fc00986f5eb687824e7b93d7a9db"),
    ("qwen2.5-1.5b-instruct-q4_k_m.gguf", 1_117_320_736, "6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e"),
    ("qwen2.5-3b-instruct-q4_k_m.gguf", 2_104_932_768, "626b4a6678b86442240e33df819e00132d3ba7dddfe1cdc4fbb18e0a9615c62d"),
    ("qwen2.5-7b-instruct-q4_k_m-00001-of-00002.gguf", 3_993_201_344, "dfce12e3862a5283ccfb88221b48480e58745165de856439950d0f22590580db"),
    ("qwen2.5-7b-instruct-q4_k_m-00002-of-00002.gguf", 689_872_288, "539cf93f78e887edea1c04e2d7d8cdaca9d01dae9c9025bcb8accbe29df3d72a"),
];

#[test]
fn every_pin_is_the_published_file_byte_for_byte() {
    assert_eq!(PINNED.len(), PUBLISHED.len());
    for (p, (name, bytes, hex)) in PINNED.iter().zip(PUBLISHED) {
        assert_eq!(&p.name[1..], name.as_bytes());
        assert_eq!(p.bytes, bytes);
        let got: alloc::string::String = p.sha256.iter().map(|b| alloc::format!("{b:02x}")).collect();
        assert_eq!(got, hex);
    }
}

#[test]
fn every_pinned_name_is_one_a_guest_can_open_and_every_tier_is_there() {
    for p in PINNED {
        let path = [&b"/models"[..], p.name].concat();
        assert_eq!(volume_name(&path), Some(p.name));
        /* The largest file the volume holds is 6,378,981,576 bytes. */
        assert!(p.bytes > 0 && p.bytes < 6_378_981_576);
    }
    for tier in ["small", "medium", "large", "xlarge"] {
        assert!(PINNED.iter().any(|p| p.tier == tier), "{tier}");
    }
}
