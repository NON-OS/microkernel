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

//! A DDI trained end to end: the trainer drives the Intel registers and
//! the sink behind them sees the link, pattern and drive they set.

use crate::dp::caps::Caps;
use crate::dp::lane::Drive;
use crate::dp::train::train;
use crate::engine::aux_regs::Ddi;
use crate::engine::ddi::{buf_ctl, buf_trans, entry, tp_ctl, NORMAL, PATTERN_MASK, TRANS_SHIFT};
use crate::engine::dpll::{DPLL_CTRL1, DPLL_CTRL2};
use crate::engine::port::Port;
use crate::model::sink::{sink, Sink};
use crate::model::{Gen9, Ticks};

pub(super) fn board(ddi: Ddi) -> (Gen9, Sink) {
    let mut m = Gen9::new(1);
    let s = sink(vec![]);
    m.sink = Some((ddi, s.clone()));
    m.set(buf_trans(ddi as u32, 0), 0x0000_2016);
    (m, s)
}

#[test]
fn ddi_b_trains_at_hbr2_on_four_lanes_through_its_registers() {
    let (mut m, s) = board(Ddi::B);
    let clock = Ticks::new();
    let mut p = Port { mmio: &mut m, ddi: Ddi::B, dpll: 1, enhanced: true, clock: &clock };
    let caps = Caps::read(&mut p).unwrap().unwrap();
    let t = train(&mut p, &caps, (5400, 4)).unwrap();
    assert_eq!((t.mbps, t.lanes, t.drive), (5400, 4, Drive { swing: 1, pre: 1 }));
    assert_eq!((m.get(DPLL_CTRL1) >> 6) & 0x3F, 0b000_001, "DPLL 1 at 5.4 Gbps");
    assert_eq!((m.get(DPLL_CTRL2) >> 3) & 7, 0b011, "DDI B on DPLL 1");
    assert_eq!(m.get(buf_ctl(1)) >> TRANS_SHIFT & 0xF, entry(t.drive));
    assert_eq!(m.get(tp_ctl(1)) & PATTERN_MASK, NORMAL);
    assert_eq!(s.borrow().dpcd[0x102], 0);
}

#[test]
fn a_slow_sink_trains_slower_and_each_failed_rate_is_taken_down() {
    let (mut m, s) = board(Ddi::C);
    s.borrow_mut().lock_mbps = 1620;
    let clock = Ticks::new();
    let mut p = Port { mmio: &mut m, ddi: Ddi::C, dpll: 2, enhanced: true, clock: &clock };
    let caps = Caps::read(&mut p).unwrap().unwrap();
    assert_eq!(train(&mut p, &caps, (5400, 4)).map(|t| t.mbps), Ok(1620));
    assert!(m.writes.iter().filter(|w| w.0 == buf_ctl(2) && w.1 == 0).count() >= 2);
}
