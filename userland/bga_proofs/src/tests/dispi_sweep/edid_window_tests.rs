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

//! The EDID window at BAR2+0 with any bytes in it: copied out only when it
//! opens with the base block's magic, the base block and as many extensions
//! as it counts up to the window, byte for byte; and the panel made of it
//! holds only modes this adapter can show, each once.

use nonos_devmodel::FakeBar;

use crate::constants::EDID_WINDOW;
use crate::display::fits;

use super::oracle::makes;
use crate::edid::{read, Panel};
use crate::model::VRAM;
use crate::tests::wire_sweep::rng::Rng;

const MAGIC: [u8; 8] = [0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0];

#[test]
fn any_edid_window_is_copied_by_its_own_count_and_read_as_a_panel_that_fits() {
    let mut r = Rng(0x6564_7701);
    for i in 0..3_000 {
        let bar = FakeBar::new(EDID_WINDOW);
        let bytes: Vec<u8> = (0..EDID_WINDOW).map(|_| r.next() as u8).collect();
        bytes.iter().enumerate().for_each(|(k, b)| bar.present8(k, *b));
        if r.below(4) != 0 {
            MAGIC.iter().enumerate().for_each(|(k, b)| bar.present8(k, *b));
        }
        let mut out = [0u8; EDID_WINDOW];
        // SAFETY: the window maps EDID_WINDOW readable bytes.
        let n = unsafe { read(bar.base(), &mut out) };
        let seen: Vec<u8> = (0..EDID_WINDOW).map(|k| bar.wrote8(k)).collect();
        let want =
            if seen[..8] == MAGIC { ((1 + seen[126] as usize) * 128).min(EDID_WINDOW) } else { 0 };
        assert_eq!(n, want, "case {i}");
        assert_eq!(out[..n], seen[..n], "case {i}: not the window's bytes");
        let p = Panel::read(&out[..n], |w, h| fits(VRAM, w, h));
        let modes = &p.modes[..p.len];
        for (k, m) in modes.iter().enumerate() {
            assert!(makes(VRAM as u64, m.width, m.height), "case {i}: {}x{}", m.width, m.height);
            assert!(
                !modes[..k].iter().any(|o| (o.width, o.height) == (m.width, m.height)),
                "case {i}"
            );
        }
    }
}
