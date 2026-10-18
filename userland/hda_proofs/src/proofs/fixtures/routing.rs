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

//! The widgets between a pin and its DAC, and a layout that routes through
//! them.

use super::{dac, pin, set_conn, w, Desc, PCM_48K16, PIN_OUT_WCAPS};
use crate::controller::codec::widget::Widget;

pub fn mixer(nid: u8, conn: &[u8]) -> Widget {
    let mut m = w(nid, 0x0020_010b);
    m.amp_in = 0x8000_0000;
    set_conn(&mut m, conn);
    m
}

pub fn selector(nid: u8, conn: &[u8]) -> Widget {
    let mut s = w(nid, 0x0030_0101);
    set_conn(&mut s, conn);
    s
}

/// An older Realtek layout: each pin reaches its DAC through a mixer that
/// also sums the analog loopback 0x0b.
pub fn alc269_mixers() -> Desc {
    Desc {
        vendor: 0x10ec_0269,
        subsystem: 0x1025_0000,
        gpio: 0x4000_0002,
        widgets: vec![
            dac(0x02, PCM_48K16),
            dac(0x03, PCM_48K16),
            mixer(0x0b, &[0x18, 0x19]),
            mixer(0x0c, &[0x02, 0x0b]),
            mixer(0x0d, &[0x03, 0x0b]),
            pin(0x14, PIN_OUT_WCAPS, 0x0001_0014, 0x9017_0110, &[0x0c, 0x0d]),
            pin(0x15, PIN_OUT_WCAPS, 0x0001_001c, 0x0221_1020, &[0x0c, 0x0d]),
        ],
    }
}
