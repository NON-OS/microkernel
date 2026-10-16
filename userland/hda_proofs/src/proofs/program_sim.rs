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

//! The ALC236 and its kin programmed on a simulated link, and the verbs that
//! came back off the ring, for the program proofs to read.

use std::sync::atomic::Ordering;

use crate::constants::VERB_SET_AMP_GAIN_MUTE;
use crate::controller::codec::plan::{plan, Class};
use crate::controller::codec::program::program;
use crate::controller::codec::walk::walk;
use crate::controller::{compose_verb, compose_verb_long};
use crate::proofs::fixtures::{sim_codec, Desc};
use crate::sim::{self, Sim};

pub const TAG: u8 = 1;

pub fn programmed(d: &Desc, plugged: bool) -> (Sim, Vec<u32>) {
    let s = sim::start(vec![(0, sim_codec(d))]);
    s.coefs.lock().unwrap().insert((0, 0x20, 0x10), 0x0220);
    s.plugged.store(plugged, Ordering::Release);
    let mut link = s.link();
    let c = walk(&mut link, 0).unwrap().unwrap();
    let Class::Analog(p) = plan(&c) else { panic!("no plan") };
    let before = s.sent().len();
    let sensed = program(&mut link, &c, &p, TAG).expect("programming failed");
    assert_eq!(sensed, plugged, "the jack was not read");
    let sent = s.sent()[before..].to_vec();
    (s, sent)
}

pub fn at(sent: &[u32], cmd: u32) -> usize {
    sent.iter().position(|&c| c == cmd).unwrap_or_else(|| panic!("{cmd:#010x} was never sent"))
}

pub fn short(nid: u8, verb: u16, payload: u16) -> u32 {
    compose_verb(0, nid, verb, payload)
}

pub fn amp(nid: u8, payload: u16) -> u32 {
    compose_verb_long(0, nid, VERB_SET_AMP_GAIN_MUTE as u16, payload)
}
