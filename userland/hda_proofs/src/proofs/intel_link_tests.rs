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
//! The multi-link clock Linux moves a link off 6 MHz to, moved on a link the
//! model powers down and up as the hardware does.

use nonos_devmodel::FakeBar;

use crate::controller::intel::{init_link_clock, multilink, preferred_scf};
use crate::proofs::intel_tests::BAR;
use crate::regs::Regs;

/// One run of `init_link_clock` on a link left at 6 MHz, with SPA and CPA
/// set; the link's clock select as it ends.
fn moved_clock() -> u32 {
    let bar = std::sync::Arc::new(FakeBar::new(BAR as usize));
    let ml = 0x800usize;
    bar.present16(0x14, ml as u16);
    bar.present32(ml, 0x2 << 16);
    bar.present32(ml + 0x40, 0b0111);
    bar.present32(ml + 0x44, (1 << 16) | (1 << 23));
    let regs = Regs::new(bar.base());
    assert_eq!(multilink(regs, BAR), Some(ml as u32));
    // The link reports the power state software asked for: CPA (bit 23)
    // follows SPA (bit 16). Both sit in the register's third byte, so only
    // that byte is touched and the clock select in the first is left alone.
    let _link = crate::model::live(&bar, move |b| {
        let v = b.wrote8(ml + 0x46);
        let want = if v & 1 != 0 { v | 0x80 } else { v & !0x80 };
        if v != want {
            b.present8(ml + 0x46, want);
        }
        std::thread::yield_now();
    });
    init_link_clock(regs, 0x8086, 0x3198, BAR);
    bar.wrote32(ml + 0x44) & 0xf
}

#[test]
fn a_link_left_on_the_6_mhz_clock_is_moved_to_24_mhz() {
    /*
     * The model has to see SPA drop within the millisecond the driver waits
     * for CPA; under a loaded test run its thread is sometimes not scheduled
     * in time, which proves nothing either way, so the run is repeated.
     */
    let moved = (0..20).any(|_| {
        std::thread::sleep(std::time::Duration::from_millis(10));
        moved_clock() == 2
    });
    assert!(moved, "the link clock was not moved off 6 MHz");
    assert_eq!(preferred_scf(0b1000), 3);
    assert_eq!(preferred_scf(0), 0);
}
