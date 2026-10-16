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

//! The amps on the ALC236's path, read back off the ring: muted at their
//! 0 dB step until a stream plays, opened by its start, closed by its stop.

use crate::constants::{AMP_MUTE, AMP_OUT_UNMUTE};
use crate::controller::codec::amp::{out_amp, set_outputs, zero_db};
use crate::controller::codec::plan::{plan, Class};
use crate::controller::codec::program::program;
use crate::controller::codec::walk::walk;
use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::proofs::program_sim::{amp, at, programmed, TAG};
use crate::sim;

#[test]
fn every_amp_on_the_path_is_left_muted_at_its_zero_db_step_until_a_stream_plays() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    at(&sent, amp(0x02, AMP_OUT_UNMUTE | AMP_MUTE | 0x57));
    at(&sent, amp(0x14, AMP_OUT_UNMUTE | AMP_MUTE));
    at(&sent, amp(0x21, AMP_OUT_UNMUTE | AMP_MUTE));
    for nid in [0x02u8, 0x14, 0x21] {
        let open = sent.iter().any(|&c| {
            c >> 8 == amp(nid, 0) >> 8 && c & 0x8000 != 0 && c & u32::from(AMP_MUTE) == 0
        });
        assert!(!open, "node {nid:#04x} was opened before any stream ran");
    }
    assert!(!sent.contains(&amp(0x02, AMP_OUT_UNMUTE | 0x7f)), "a gain past the amp's steps");
    assert_eq!(zero_db(0x0005_7057), 0x57);
    assert_eq!(zero_db(0x0000_0060), 0, "an offset past the step count is held to it");
}

#[test]
fn a_stream_opens_every_amp_on_the_path_at_zero_db_and_its_stop_closes_them() {
    let d = alc236_hp();
    let s = sim::start(vec![(0, sim_codec(&d))]);
    let mut link = s.link();
    let c = walk(&mut link, 0).unwrap().unwrap();
    let Class::Analog(p) = plan(&c) else { panic!("no plan") };
    program(&mut link, &c, &p, TAG).unwrap();
    let before = s.sent().len();
    set_outputs(&mut link, &c, &p, true).unwrap();
    let opened = s.sent()[before..].to_vec();
    at(&opened, amp(0x02, AMP_OUT_UNMUTE | 0x57));
    at(&opened, amp(0x14, AMP_OUT_UNMUTE));
    at(&opened, amp(0x21, AMP_OUT_UNMUTE));
    let before = s.sent().len();
    set_outputs(&mut link, &c, &p, false).unwrap();
    let closed = s.sent()[before..].to_vec();
    at(&closed, amp(0x02, AMP_OUT_UNMUTE | AMP_MUTE | 0x57));
    at(&closed, amp(0x14, AMP_OUT_UNMUTE | AMP_MUTE));
    at(&closed, amp(0x21, AMP_OUT_UNMUTE | AMP_MUTE));
    assert_eq!(out_amp(0x0005_7057, true), AMP_OUT_UNMUTE | 0x57);
    assert_eq!(out_amp(0x0005_7057, false), AMP_OUT_UNMUTE | AMP_MUTE | 0x57);
}
