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
//! The DDI buffer on and off (DDI_BUF_CTL, Vol 2c pp. 353 to 355; Vol 12
//! steps 7f, 7g and 3e, pp. 144, 148).

use crate::model::clock::Steps;
use crate::model::regs::Regs;
use nonos_intel_xelp_display::ddi::buf::{disable, enable};
use nonos_intel_xelp_display::regs::clka::Combo;

const BUF_C: u32 = 0x64200;

#[test]
fn buffer_on_off() {
    let mut m = Regs::new(Some(1));
    m.set(BUF_C, (1 << 7) | (1 << 16) | (1 << 29) | 1);
    enable(&mut m, &Steps::new(1), Combo::C, 2).unwrap();
    // Enable, x2, reversal and override off, Init Display Detected kept.
    assert_eq!(m.get(BUF_C), (1 << 31) | (0b001 << 1) | 1);
    disable(&mut m, &Steps::new(1), Combo::C).unwrap();
    assert_eq!(m.get(BUF_C), (1 << 7) | (0b001 << 1) | 1);
}

#[test]
fn buffer_that_stays_idle_is_turned_off() {
    let mut m = Regs::new(Some(0));
    m.set(BUF_C, 1 << 7);
    m.stuck.insert((BUF_C, 7));
    assert!(enable(&mut m, &Steps::new(100), Combo::C, 4).is_err());
    assert_eq!(m.get(BUF_C) & (1 << 31), 0);
    assert!(enable(&mut m, &Steps::new(1), Combo::C, 3).is_err());
}

#[test]
fn port_widths() {
    // DP Port Width: 000b x1, 011b x4 (Vol 2c p. 355).
    for (lanes, code) in [(1, 0b000), (4, 0b011)] {
        let mut m = Regs::new(Some(0));
        m.set(0x64000, 1 << 7);
        enable(&mut m, &Steps::new(1), Combo::A, lanes).unwrap();
        assert_eq!((m.get(0x64000) >> 1) & 7, code);
    }
}
