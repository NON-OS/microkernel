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

//! Input amps past the sixteenth input, on the path search's side.
//!
//! SET_AMP_GAIN_MUTE names an input amp in four bits (11:8), so input 16 and
//! above cannot be addressed: the index wraps, and `(17 << 8)` opens input 1
//! instead. A mixer whose input amps are on the path routes through the
//! input it opens, so the search must not take a hop it cannot open.

use crate::constants::WCAP_IN_AMP;
use crate::controller::codec::path::find;
use crate::controller::codec::widget::Widget;
use crate::proofs::fixtures::{codec, dac, mixer, pin, selector, Desc, PCM_48K16};
use crate::proofs::fixtures::{NC, PIN_IN_WCAPS, PIN_OUT_WCAPS};

/// A speaker pin into `hub` (0x0c), whose input `at` of `n` is the DAC and
/// every other input the microphone pin 0x18.
pub fn through(hub: fn(u8, &[u8]) -> Widget, n: usize, at: usize) -> Desc {
    let mut conn = vec![0x18u8; n];
    conn[at] = 0x02;
    let speaker = pin(0x14, PIN_OUT_WCAPS, 0x10, 0x9017_0110, &[0x0c]);
    let mic = pin(0x18, PIN_IN_WCAPS, 0x24, NC, &[]);
    let widgets = vec![dac(0x02, PCM_48K16), hub(0x0c, &conn), speaker, mic];
    Desc { vendor: 0x1af4_0022, subsystem: 0, gpio: 0, widgets }
}

fn plain_mixer(nid: u8, conn: &[u8]) -> Widget {
    let mut m = mixer(nid, conn);
    m.wcaps &= !WCAP_IN_AMP;
    m
}

fn hop(d: &Desc) -> Option<u8> {
    find(&codec(d, 0), 0x14, &[]).ok().map(|p| p.sel[1])
}

#[test]
fn an_input_amp_cannot_address_input_16_so_no_route_is_taken_through_it() {
    assert_eq!(hop(&through(mixer, 17, 16)), None, "routed through an input it cannot open");
    assert_eq!(hop(&through(mixer, 24, 23)), None, "routed through an input it cannot open");
}

#[test]
fn input_15_is_the_last_an_input_amp_can_address() {
    assert_eq!(hop(&through(mixer, 16, 15)), Some(15));
    assert_eq!(hop(&through(mixer, 24, 0)), Some(0));
}

#[test]
fn without_input_amps_any_input_can_be_routed() {
    assert_eq!(hop(&through(plain_mixer, 24, 20)), Some(20), "a mixer sums every input");
    assert_eq!(hop(&through(selector, 24, 20)), Some(20), "SET_CONNECT_SEL takes 8 bits");
}
