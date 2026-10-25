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

//! The AUX channel registers carrying real DisplayPort transactions.

use crate::dp::aux::Aux;
use crate::dp::caps::Caps;
use crate::engine::aux_ch::AuxCh;
use crate::engine::aux_regs::{aux_data, Ddi};
use crate::model::sink::sink;
use crate::model::{Gen9, Ticks};
use crate::tests::dp::edid_read::read_edid;

pub(super) fn with_sink(edid: Vec<u8>) -> Gen9 {
    let mut m = Gen9::new(1);
    m.sink = Some((Ddi::B, sink(edid)));
    m
}

#[test]
fn the_dpcd_and_the_edid_come_through_the_registers() {
    let mut m = with_sink((0..128).collect());
    let clock = Ticks::new();
    let mut aux = AuxCh { mmio: &mut m, ddi: Ddi::B, clock: &clock };
    let caps = Caps::read(&mut aux).unwrap().unwrap();
    assert_eq!((caps.max_rate_mbps, caps.max_lanes), (8100, 4));
    let mut e = [0u8; 128];
    read_edid(&mut aux, 0, &mut e).unwrap();
    assert!(e.iter().enumerate().all(|(i, &b)| b == i as u8));
}

#[test]
fn request_bytes_go_most_significant_first() {
    let mut m = with_sink(vec![]);
    let clock = Ticks::new();
    let mut reply = [0u8; 20];
    let req = [0x90, 0x00, 0x00, 0x00];
    AuxCh { mmio: &mut m, ddi: Ddi::B, clock: &clock }.transfer(&req, &mut reply).unwrap();
    assert_eq!(m.writes.iter().find(|w| w.0 == aux_data(Ddi::B, 0)).unwrap().1, 0x9000_0000);
}
