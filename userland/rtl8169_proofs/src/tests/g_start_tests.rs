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

//! The 8168g start (Linux rtl_hw_start_8168g's shared steps) against a
//! part that serves ERI: the writes, in order, with their byte enables.

use std::sync::{Arc, Mutex};

use nonos_libc::logged;

use super::model::{live, window};
use super::model_eri::{eri_part, Taken};
use crate::chip::MacVersion;
use crate::hw::regs::{MISC_RXDV_GATED_EN, REG_MISC};
use crate::hw::start_8168g;
use crate::regs::Regs;

fn run(ver: u8) -> Vec<(u32, u32, u32)> {
    let bar = window();
    bar.present32(REG_MISC, MISC_RXDV_GATED_EN | 0x0000_0040);
    let taken: Taken = Arc::new(Mutex::new(Vec::new()));
    let _part = live(&bar, eri_part(&[(0xDC, 0x0000_0011)], taken.clone()));
    start_8168g(&Regs::new(bar.base()), MacVersion(ver)).expect("every ERI access completes");
    assert_eq!(bar.wrote32(REG_MISC), 0x0000_0040, "VER_{ver}: only the RXDV gate is cleared");
    let out = taken.lock().unwrap().clone();
    out
}

#[test]
fn an_8168h_gets_fifo_sizes_pause_thresholds_a_filter_reset_and_the_gate_released() {
    let want = vec![
        (0xC8, 0xF000, 0x0008_0002),
        (0xE8, 0xF000, 0x0010_0006),
        (0xCC, 0x1000, 0x38),
        (0xD0, 0x1000, 0x48),
        (0xDC, 0xF000, 0x10),
        (0xDC, 0xF000, 0x11),
        (0xC0, 0x3000, 0),
        (0xB8, 0x3000, 0),
    ];
    assert_eq!(run(46), want);
}

#[test]
fn the_8168ep_and_8117_get_their_own_pause_thresholds() {
    for ver in [51u8, 52] {
        let got = run(ver);
        assert_eq!((got[2].2, got[3].2), (0x2F, 0x5F), "VER_{ver}");
    }
}

#[test]
fn a_part_that_never_answers_eri_still_has_its_gate_released() {
    let bar = window();
    bar.present32(REG_MISC, MISC_RXDV_GATED_EN);
    let err = start_8168g(&Regs::new(bar.base()), MacVersion(40)).unwrap_err();
    assert_eq!(err, "rtl8169 ERI timeout");
    assert_eq!(bar.wrote32(REG_MISC) & MISC_RXDV_GATED_EN, 0);
    let want = "rtl8169: ERI access to 0xc8 did not complete in 10 ms\n";
    assert!(logged().iter().any(|l| l == want), "{:?}", logged());
}
