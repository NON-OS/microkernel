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

//! A vision tier's image on its stdin (capsule_terminal qwen/image.rs): only
//! a vision tier's ` < FILE` is taken, and every other line is the question
//! byte for byte, as before.

use super::ask::parse;
use super::image::{is_vision, split};
use super::tiers::TIERS;

#[path = "qwen_image_tests/frame.rs"]
mod frame;

#[test]
fn a_vision_tier_takes_a_trailing_file() {
    assert_eq!(
        split(b"vl-2b", b"What is this? < photo.jpg"),
        (&b"What is this?"[..], Some(&b"photo.jpg"[..]))
    );
    assert_eq!(split(b"vl-2b", b"< documents/a.png"), (&b""[..], Some(&b"documents/a.png"[..])));
    assert_eq!(
        split(b"vl-2b", b"Describe it <\tx.JPEG  "),
        (&b"Describe it"[..], Some(&b"x.JPEG"[..]))
    );
}

#[test]
fn anything_else_is_the_question_whole() {
    for q in [
        &b"is 2 < 3?"[..],
        b"a<b.jpg",
        b"ends with <",
        b"< two words.jpg",
        b"no file here",
        b"x < notes.txt",
        b"< .png",
    ] {
        assert_eq!(split(b"vl-2b", q), (q, None), "{q:?}");
    }
    /* Every tier but the vision ones, named, so the rule cannot read them all as vision. */
    let text = TIERS.iter().filter(|t| **t != b"vl-2b");
    assert_eq!(text.clone().count(), TIERS.len() - 1);
    for tier in text {
        assert!(!is_vision(tier), "{tier:?}");
        let q = b"What is this? < photo.jpg";
        assert_eq!(split(tier, q), (&q[..], None), "{tier:?}");
    }
}

#[test]
fn every_other_line_parses_as_it_did() {
    let ask = parse(b"qwen small What is this? < photo.jpg").unwrap();
    assert_eq!((ask.tier, ask.question), (Some(&b"small"[..]), &b"What is this? < photo.jpg"[..]));
    let ask = parse(b"qwen vl-2b What is this? < photo.jpg").unwrap();
    assert_eq!(
        ask.question, b"What is this? < photo.jpg",
        "the line itself is untouched; split is the vision step"
    );
    assert!(TIERS.contains(&&b"vl-2b"[..]));
}
