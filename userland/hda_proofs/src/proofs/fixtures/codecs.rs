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

//! The HP 15s's ALC236, the Intel display codec and QEMU's duplex codec.

use super::{dac, pin, w, Desc, HP_SUBSYSTEM, NC, PCM_48K16, PIN_IN_WCAPS, PIN_OUT_WCAPS};

pub fn alc236_hp() -> Desc {
    Desc {
        vendor: 0x10ec_0236,
        subsystem: HP_SUBSYSTEM,
        gpio: 0x4000_0003,
        widgets: vec![
            dac(0x02, PCM_48K16),
            dac(0x03, PCM_48K16),
            w(0x08, 0x0010_051b),
            w(0x09, 0x0010_051b),
            pin(0x12, 0x0040_040b, 0x0000_0020, 0x90a6_0120, &[]),
            pin(0x13, 0x0040_040b, 0x0000_0020, NC, &[]),
            pin(0x14, PIN_OUT_WCAPS, 0x0001_0014, 0x9017_0110, &[0x02, 0x03]),
            pin(0x18, PIN_IN_WCAPS, 0x0000_3724, NC, &[]),
            pin(0x19, PIN_IN_WCAPS, 0x0000_3724, 0x03a1_1030, &[]),
            pin(0x1a, PIN_OUT_WCAPS, 0x0000_3734, NC, &[0x02, 0x03]),
            pin(0x1b, PIN_OUT_WCAPS, 0x0001_3734, NC, &[0x02, 0x03]),
            pin(0x1d, 0x0040_0400, 0x0000_0020, 0x40e9_a105, &[]),
            pin(0x1e, 0x0040_0781, 0x0000_0014, NC, &[0x06]),
            w(0x20, 0x00f0_0040),
            pin(0x21, PIN_OUT_WCAPS, 0x0001_001c, 0x0221_1020, &[0x02, 0x03]),
        ],
    }
}

pub fn intel_hdmi() -> Desc {
    let mut cvt = dac(0x02, PCM_48K16);
    cvt.wcaps = 0x0000_6611;
    Desc {
        vendor: 0x8086_280d,
        subsystem: 0x8086_0101,
        gpio: 0,
        widgets: vec![
            cvt,
            pin(0x05, 0x0040_778d, 0x0b00_0094, 0x1856_0010, &[0x02]),
            pin(0x06, 0x0040_778d, 0x0b00_0094, 0x5856_0020, &[0x02]),
        ],
    }
}

pub fn qemu_duplex() -> Desc {
    let mut d = dac(0x02, 0x0002_0040);
    d.amp_out = 0x0003_4a4a;
    Desc {
        vendor: 0x1af4_0022,
        subsystem: 0x1af4_1100,
        gpio: 0,
        widgets: vec![d, pin(0x03, 0x0040_0101, 0x0000_0010, 0x0001_4010, &[0x02])],
    }
}
