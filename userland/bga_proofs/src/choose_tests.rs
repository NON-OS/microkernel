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

//! Which mode is driven: the firmware's whenever it can be served as is.

use crate::constants::{DISPI_ENABLED, DISPI_LFB_ENABLED};
use crate::dispi::{choose, Choice, Mode, Seen};

const ON: u16 = DISPI_ENABLED | DISPI_LFB_ENABLED;
const MIB: u64 = 1 << 20;
const OWN: Mode = Mode { width: 1024, height: 768, stride: 4096 };

fn ovmf() -> Seen {
    // OVMF's QemuVideoDxe leaves 1280x800x32 with its framebuffer on.
    Seen { enable: ON, xres: 1280, yres: 800, bpp: 32, virt_width: 1280, ..Seen::default() }
}

#[test]
fn the_firmwares_mode_is_kept_so_the_boot_screen_stays_up() {
    let kept = Mode { width: 1280, height: 800, stride: 5120 };
    assert_eq!(choose(ovmf(), 16 * MIB), Some(Choice::Keep(kept)));
    let wide = Seen { virt_width: 1536, ..ovmf() };
    let stride = Mode { width: 1280, height: 800, stride: 1536 * 4 };
    assert_eq!(choose(wide, 16 * MIB), Some(Choice::Keep(stride)), "stride from VIRT_WIDTH");
}

#[test]
fn a_mode_this_driver_cannot_serve_is_replaced_by_its_own() {
    let cases = [
        Seen { enable: DISPI_ENABLED, ..ovmf() },
        Seen { enable: 0, ..ovmf() },
        Seen { bpp: 16, ..ovmf() },
        Seen { virt_width: 1024, ..ovmf() },
        Seen { x_offset: 8, ..ovmf() },
        Seen { y_offset: 800, ..ovmf() },
        Seen { xres: 0, ..ovmf() },
        Seen::default(),
    ];
    for s in cases {
        assert_eq!(choose(s, 16 * MIB), Some(Choice::Set(OWN)), "{s:?}");
    }
}

#[test]
fn vram_bounds_both_the_kept_mode_and_the_drivers_own() {
    // 1280x800x4 is 4,096,000 bytes; 1024x768x4 is 3 MiB.
    assert_eq!(
        choose(ovmf(), 4_096_000),
        Some(Choice::Keep(Mode { width: 1280, height: 800, stride: 5120 }))
    );
    assert_eq!(choose(ovmf(), 4_095_999), Some(Choice::Set(OWN)));
    assert_eq!(choose(ovmf(), 3 * MIB), Some(Choice::Set(OWN)));
    assert_eq!(choose(ovmf(), 3 * MIB - 1), None);
}
