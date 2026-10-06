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

//! Handing an output stream descriptor a buffer and starting it.
//!
//! A stream descriptor is a bus master with its own address registers. Once
//! RUN is set it reads whatever those registers point at, at the rate the
//! format register implies, and it does not stop for a driver that changes its
//! mind. Every value below has to be right before that happens, and the
//! specification says as much: the buffer registers are not to be touched
//! while the engine is running.

use nonos_devmodel::FakeBar;

use crate::constants::{SD_BDPL, SD_BDPU, SD_CBL, SD_LVI};
use crate::controller::bdl::{N_PERIODS, RING_BYTES};
use crate::controller::codec::format::PLAYBACK;
use crate::controller::stream_run::{run, StreamRun};
use crate::model::{descriptor, WINDOW};
use crate::regs::Regs;

/// The first output stream on a four-in, four-out controller, and the block
/// the layout puts it in.
pub const INDEX: u16 = 4;
pub const OFF: u32 = 0x100;
pub const TAG: u8 = 1;
pub const BDL_DEV: u64 = 0x0000_0005_1234_0000;
pub const SAMPLE_DEV: u64 = 0x0000_0006_abcd_0000;
pub const BDL_BYTES: usize = 128;

pub fn play(bar: &FakeBar, bdl: &FakeBar) {
    let r = StreamRun {
        desc: descriptor(INDEX),
        tag: TAG,
        format: PLAYBACK,
        bdl_va: bdl.base(),
        bdl_dev: BDL_DEV,
        sample_dev: SAMPLE_DEV,
        bytes: RING_BYTES as u32,
    };
    assert!(run(Regs::new(bar.base()), r).is_ok(), "the stream did not finish its reset");
}

#[test]
fn the_engine_is_pointed_at_the_descriptor_list_in_both_address_halves() {
    let (bar, bdl) = (FakeBar::new(WINDOW), FakeBar::new(BDL_BYTES));
    play(&bar, &bdl);
    assert_eq!(bar.wrote32((OFF + SD_BDPL) as usize), BDL_DEV as u32);
    assert_eq!(bar.wrote32((OFF + SD_BDPU) as usize), (BDL_DEV >> 32) as u32);
    assert_eq!(bar.wrote32((OFF + SD_CBL) as usize), RING_BYTES as u32, "wrong ring length");
    assert_eq!(bar.wrote16((OFF + SD_LVI) as usize), (N_PERIODS - 1) as u16, "wrong last index");
}

#[test]
fn a_bidirectional_engine_is_set_to_run_as_output() {
    use crate::constants::{SDCTL2_DIR_OUT, SD_CTL};
    use crate::controller::STREAM_BIDI;
    let (bar, bdl) = (FakeBar::new(WINDOW), FakeBar::new(BDL_BYTES));
    let mut desc = descriptor(INDEX);
    desc.kind = STREAM_BIDI;
    let r = StreamRun {
        desc,
        tag: TAG,
        format: PLAYBACK,
        bdl_va: bdl.base(),
        bdl_dev: BDL_DEV,
        sample_dev: SAMPLE_DEV,
        bytes: RING_BYTES as u32,
    };
    assert!(run(Regs::new(bar.base()), r).is_ok());
    assert_eq!(bar.wrote8((OFF + SD_CTL) as usize + 2), (TAG << 4) | SDCTL2_DIR_OUT);
}

#[test]
fn a_running_engine_is_stopped_before_its_reset() {
    use crate::constants::{SDCTL_RUN, SD_CTL};
    use crate::controller::stream_run::reset;
    let bar = FakeBar::new(WINDOW);
    let ctl = (OFF + SD_CTL) as usize;
    bar.present8(ctl, SDCTL_RUN);
    assert!(reset(Regs::new(bar.base()), OFF).is_ok());
    assert_eq!(bar.wrote8(ctl) & SDCTL_RUN, 0, "the engine was reset while running");
}
