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

//! A mode against the adapter's video memory, which VIDEO_MEMORY_64K
//! reports, as well as against BAR0. A mode past the memory scans out what
//! is not there; it is refused by name before a register is written.

use super::adapter::{adapter, found, MODE, MODE_BYTES, REG_LEN};
use super::attempt::run;
use crate::constants::{DISPI_INDEX_VIDEO_MEMORY_64K, DISPI_INDEX_XRES};
use crate::dispi::dispi_off;
use crate::error::BgaError;

#[test]
fn a_mode_past_the_adapters_video_memory_is_refused_by_name_before_it_is_set() {
    let video = dispi_off(DISPI_INDEX_VIDEO_MEMORY_64K) as usize;
    // 48 units of 64 KiB hold 1024x768x32 exactly.
    for units in [0, 1, 16, 47, u16::MAX] {
        let a = adapter(REG_LEN, MODE_BYTES);
        a.regs.present16(video, units);
        let r = run(found(MODE_BYTES));
        if units == u16::MAX {
            assert!(r.is_ok(), "4 GiB of video memory refused");
            continue;
        }
        assert_eq!(r.err(), Some(BgaError::VideoMemoryTooSmall), "{units} units");
        assert_eq!(a.regs.wrote16(dispi_off(DISPI_INDEX_XRES) as usize), 0, "mode set");
        assert_eq!(a.fb.wrote32(0), 0, "drawn into a mode the adapter cannot hold");
    }
    let a = adapter(REG_LEN, MODE_BYTES);
    a.regs.present16(video, 48);
    let d = run(found(MODE_BYTES)).unwrap_or_else(|e| panic!("refused: {e:?}"));
    let m = d.server.display.mode;
    assert_eq!((m.width, m.height), MODE);
}
