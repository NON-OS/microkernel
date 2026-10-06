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
//! Staying inside the ring that was actually allocated.

use nonos_devmodel::run;

use crate::constants::{RIRBSTS, RIRBSTS_INTFL};
use crate::model::{corb_engine, rings, window};
use crate::proofs::verb_tests::{link_over, CMD, RESPONSE};

#[test]
fn the_write_pointer_wraps_within_the_ring_rather_than_running_off_the_end() {
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let _controller = run(&bar, corb_engine);
    let mut link = link_over(&bar, corb.base(), rirb.base());
    for i in 0..256u32 {
        assert!(link.send(CMD + i).is_ok(), "verb {i}");
    }
    assert_eq!(bar.wrote16(crate::constants::CORBWP as usize), 0, "the pointer left the 256-entry ring");
    assert_eq!(corb.wrote32(0), CMD + 255, "the last command did not wrap into slot zero");
    assert_eq!(corb.wrote32(4), CMD, "the first command was overwritten early");
}

#[test]
fn the_response_interrupt_flag_is_acknowledged_after_every_answer() {
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let _controller = run(&bar, corb_engine);
    let mut link = link_over(&bar, corb.base(), rirb.base());
    assert!(link.send(CMD).is_ok());
    assert_eq!(bar.wrote8(RIRBSTS as usize), RIRBSTS_INTFL);
}
