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

//! The Settings row and a vision tier.

use crate::qwen_tier::{shown, step};

/*
 * A plain `qwen` runs the chosen tier with no image, so the row never offers
 * a vision tier: vl-2b is opened by name, with its picture. One already in
 * the store still reads as itself, and steps on to a text tier.
 */
#[test]
fn the_row_never_offers_a_vision_tier() {
    let mut at: &[u8] = b"";
    for _ in 0..40 {
        at = step(at, 1);
        assert_ne!(at, b"vl-2b");
    }
    assert_eq!(step(b"qwen3.6-27b", 1), b"coder-1.5b");
    assert_eq!(step(b"coder-1.5b", -1), b"qwen3.6-27b");
    assert_eq!(shown(b"vl-2b"), b"Qwen3-VL 2B (vision)");
    assert_eq!(step(b"vl-2b", 1), b"small");
}
