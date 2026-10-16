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

//! A mode the adapter does not take, end to end: an adapter that clamps the
//! width it is given, as one with less video memory than the mode needs
//! does. The read-back fails the attempt by name before the clear, so
//! nothing is drawn into a mode the adapter is not showing.

use nonos_devmodel::run as live;
use nonos_libc::given_back;

use crate::adapter::{adapter, found, DEVICE, MODE_BYTES, REG_LEN};
use crate::constants::DISPI_INDEX_XRES;
use crate::dispi::dispi_off;
use crate::error::BgaError;
use crate::setup::run;

/// The widest mode the clamping adapter shows.
const MAX_XRES: u16 = 800;

#[test]
fn a_mode_the_adapter_clamps_fails_by_name_with_nothing_drawn() {
    let xres = dispi_off(DISPI_INDEX_XRES) as usize;
    // The adapter is a thread. A run in which the driver read XRES back
    // before the adapter clamped it proves nothing, so it is run again.
    for _ in 0..100 {
        let a = adapter(REG_LEN, MODE_BYTES);
        a.regs.present16(xres, u16::MAX);
        let _clamp = live(&a.regs, move |b| {
            if b.wrote16(xres) > MAX_XRES {
                b.present16(xres, MAX_XRES);
            }
        });
        // Up and clamping before the attempt starts, which takes less time
        // than a thread does to start.
        while a.regs.wrote16(xres) != MAX_XRES {
            std::thread::yield_now();
        }
        let before = given_back().len();
        if !matches!(run(found(MODE_BYTES)), Err(BgaError::ModeNotTaken)) {
            continue;
        }
        let drawn = (0..MODE_BYTES).step_by(4).find(|&o| a.fb.wrote32(o) != 0);
        assert_eq!(drawn, None, "drawn into a mode the adapter did not take");
        let this_run = given_back().get(before..).and_then(<[_]>::last).copied();
        assert_eq!(this_run, Some(("device_release", DEVICE)), "the claim was kept");
        return;
    }
    panic!("in 100 runs under a clamping adapter no attempt was refused as ModeNotTaken");
}
