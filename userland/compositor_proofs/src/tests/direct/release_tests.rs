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

use alloc::vec::Vec;

use crate::frame_pacer::trace::line::direct;
use crate::frame_pacer::trace::record::FrameRecord;
use crate::state::retire::Retiring;
use crate::swapchain::present::{Present, INDEX_OPAQUE};
use crate::swapchain::swapchain_damage::Damage;

/// What frame_pacer/release.rs asks: a buffer is shown while a layer
/// shows it or the plane still scans it.
fn release(r: &mut Retiring, layer: u64, plane: Option<u64>) -> Vec<u64> {
    let mut out = Vec::new();
    let shown = |h: u64| plane == Some(h) || h == layer;
    r.release_unshown(shown, |h| out.push(h));
    out
}

#[test]
fn a_buffer_on_the_plane_waits_for_the_flip_that_replaced_it() {
    let (a, b) = (0xA, 0xB);
    let mut r = Retiring::new();
    // B presented: the layer shows B, A retires, but SCANOUT(B) was refused
    // and the plane still scans A until the composed FLUSH lands.
    assert!(r.hold(a));
    assert!(release(&mut r, b, Some(a)).is_empty(), "A is still on the plane");
    // The FLUSH landed (plane None) or SCANOUT(B) answered (plane B).
    assert_eq!(release(&mut r, b, Some(b)), [a]);
}

#[test]
fn a_direct_frame_still_prints_a_row_and_says_so() {
    let line = direct(12, 0x2_0000_0011);
    assert_eq!(line, "[FTRACE] # direct frame=12 handle=0x200000011\n");
    assert_eq!(FrameRecord::default().direct, 0, "a composed frame names no buffer");
}

#[test]
fn the_opaque_bit_rides_in_the_index_word_and_leaves_the_index_alone() {
    let damage = Damage { x: 0, y: 0, width: 1, height: 1 };
    let p = Present { handle: 9, z: 2, index: 1, damage, opaque: true };
    let got = crate::requests::present::present(&p.encode()).expect("decodes");
    assert!(got.opaque);
    assert_eq!(got.index, 1);
    let q = Present { opaque: false, ..p };
    assert!(!crate::requests::present::present(&q.encode()).expect("decodes").opaque);
    assert_eq!(INDEX_OPAQUE, crate::requests::present::INDEX_OPAQUE);
}
