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

//! A codec graph built to defeat the path search.
//!
//! The graph is what codec firmware reports. The search deepens to six
//! widgets over up to 24 connections each, and a pin with no route used to
//! run it to the end: mixers that each list 24 other mixers, and no DAC,
//! made that millions of visits a pin, for every output pin, with no
//! deadline, while the bring-up held a core. Now each pin's search has a
//! budget, and running it out is a named outcome, not a missing route.

use std::sync::mpsc;
use std::time::Duration;

use crate::controller::codec::path::{find, Miss};
use crate::controller::codec::plan::{plan, Class};
use crate::proofs::fixtures::{codec, dac, mixer, pin, Desc, PCM_48K16, PIN_OUT_WCAPS};

const SPEAKER: u32 = 0x9017_0110;

/// Sixteen speaker pins into a ring of 79 mixers, each listing the next 24.
/// With `with_dac`, the first mixer's input 15 is a DAC instead.
pub fn maze(with_dac: bool) -> Desc {
    let mixers: Vec<u8> = (0x20..0x6f).collect();
    let mut widgets = if with_dac { vec![dac(0x02, PCM_48K16)] } else { Vec::new() };
    for (i, &m) in mixers.iter().enumerate() {
        let mut conn: Vec<u8> = (1..=24).map(|k| mixers[(i + k) % mixers.len()]).collect();
        if with_dac && i == 0 {
            conn[15] = 0x02;
        }
        widgets.push(mixer(m, &conn));
    }
    for p in 0x10..0x20 {
        widgets.push(pin(p, PIN_OUT_WCAPS, 0x10, SPEAKER, &mixers[..24]));
    }
    Desc { vendor: 0x1af4_0022, subsystem: 0, gpio: 0, widgets }
}

/// `plan` on `d`, or `None` if it has not finished in `secs` seconds.
fn planned_within(d: Desc, secs: u64) -> Option<Class> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || tx.send(plan(&codec(&d, 0))));
    rx.recv_timeout(Duration::from_secs(secs)).ok()
}

#[test]
fn a_maze_of_mixers_with_no_dac_is_given_up_on_by_name_in_bounded_time() {
    let class = planned_within(maze(false), 10).expect("the path search ran past 10 s");
    assert_eq!(class, Class::OverBudget, "a spent budget read as a codec with no route");
    let speaker = find(&codec(&maze(false), 0), 0x10, &[]);
    assert_eq!(speaker, Err(Miss::OverBudget));
}

#[test]
fn a_dac_inside_the_maze_is_still_found() {
    let Some(Class::Analog(p)) = planned_within(maze(true), 10) else { panic!("no plan") };
    assert!(p.iter().all(|o| o.path.dac() == 0x02 && o.path.len == 3), "{p:?}");
    assert_eq!(p.iter().count(), 6, "every output slot takes a pin");
}
