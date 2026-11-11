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

//! About's prose broken by the toolkit's `wrap_hard` where its private
//! `prose_wrap::split` broke it. Below the widest glyph the old byte-wise
//! fallback kept a space as a line of its own, which `wrap_hard` drops; About's
//! window floor keeps its prose far wider than that.

use nonos_toolkit::widgets::text::{width, wrap_hard};

use crate::about::ui::metrics::metrics;
use crate::about_data::trust::STATUS;
use crate::about_old_wrap::{old_lines, valid};

#[path = "../../capsule_about/src/about/ui/screens/verify_items.rs"]
mod verify_items;

const AGREE_FROM: u32 = 11;

const SAMPLES: [&[u8]; 6] = [
    b"0xffff_ffff_8000_0000 Ed25519 + ML-DSA-65 (hybrid)",
    b"trust-anchor -> publisher -> capsule",
    b"caf\xc3\xa9 \xc3\xbcn\xc3\xafcode \xe2\x9c\x93 overlongtokenwithoutanyspaceinsideit",
    b"MIT OR Apache-2.0",
    b"a",
    b"",
];

fn wrapped() -> impl Iterator<Item = &'static [u8]> {
    verify_items::ITEMS.into_iter().map(|(_, why)| why).chain([STATUS]).chain(SAMPLES)
}

#[test]
fn every_wrapped_run_breaks_where_the_old_split_broke_it() {
    let px = metrics().body_px;
    for b in wrapped() {
        let text = valid(b);
        for w in AGREE_FROM..=width(text, px) + 4 {
            let new: Vec<&[u8]> =
                wrap_hard(text, w, px).into_iter().map(|(s, e)| &b[s..e]).collect();
            assert_eq!(new, old_lines(b, w), "{text:?} in {w}");
        }
    }
}
