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

//! The cover rule against the page-by-page model (L79), over seeded cases of
//! six BARs each, of every kind and size including none, behind a random
//! count. Floors on what the cases met keep the sweep from passing on
//! nothing: held and not held, unread BARs that would have held had they
//! been read, and live ones a page short of the framebuffer.

use super::cover_find::MMIO;
use super::cover_gen::{bar, frame};
use super::cover_model::{bits, touched, Lcg};
use super::cover_rule::{holds, pages};

#[test]
fn seeded_bars_of_every_kind_are_held_as_their_pages_say() {
    let mut g = Lcg(0x4C37_39C0_FB00_0079);
    let (mut held, mut apart, mut decoy, mut edge) = (0u32, 0u32, 0u32, 0u32);
    for case in 0..20_000 {
        let fb = frame(&mut g);
        let count = g.below(7) as usize;
        let bars: Vec<_> = (0..6).map(|_| bar(&mut g, fb)).collect();
        let fm = touched(fb.0, fb.1);
        let live = |i: usize| i < count && bars[i].2 == MMIO && bars[i].1 > 0;
        let lit = (0..6).filter(|&i| live(i)).fold(0, |m, i| m | touched(bars[i].0, bars[i].1));
        let dead = (0..6).filter(|&i| !live(i));
        let dark = dead.fold(0, |m, i| m | bits(pages(bars[i].0, bars[i].1)));
        let want = fm & lit != 0;
        assert_eq!(holds(fb, &bars, count, MMIO), want, "case {case}: {fb:x?} {bars:x?}");
        held += u32::from(want);
        apart += u32::from(!want);
        decoy += u32::from(!want && fm & dark != 0);
        edge += u32::from(!want && (fm << 1 | fm >> 1) & lit != 0);
    }
    assert!(held > 5_000 && apart > 10_000, "held {held}, not held {apart}");
    assert!(decoy > 5_000 && edge > 2_000, "unread BARs {decoy}, edges {edge}");
}
