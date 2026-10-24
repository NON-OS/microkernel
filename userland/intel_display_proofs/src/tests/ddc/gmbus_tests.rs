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

//! DDC/CI over GMBUS against the model's monitor on DDI B's pins: a read
//! and a set arrive byte for byte at 0x37 and at nothing else, the pins are
//! let go after each; the wrong pins, DDI A and a spoiled reply give nothing.

use super::caps_rig::known;
use crate::ddc::error::DdcError;
use crate::ddc::reply::Reading;
use crate::ddc::transact::read;
use crate::ddc::transact_write::write;
use crate::engine::gmbus_regs::{CYCLE_STOP, GMBUS0, GMBUS1, SW_CLR_INT, SW_RDY};
use crate::engine::gmbus_xfer::ddc::GmbusDdc;
use crate::model::mccs::Mccs;
use crate::model::{Gen9, Ticks};

fn monitor_on(pin: u32) -> Gen9 {
    let mut m = Gen9::new(1);
    m.ddc.mccs = Some((pin, Mccs::with(&[(0x10, 70, 100)])));
    m
}

fn ask(m: &mut Gen9, ddi: u32, code: u8) -> Result<Reading, DdcError> {
    let t = Ticks::new();
    read(&mut GmbusDdc { mmio: m, clock: &t, ddi }, &t, &mut known(), code)
}

#[test]
fn a_read_and_a_set_reach_the_monitor_at_0x37_alone() {
    let (mut m, t, mut p) = (monitor_on(5), Ticks::new(), known());
    let mut bus = GmbusDdc { mmio: &mut m, clock: &t, ddi: 1 };
    assert_eq!(read(&mut bus, &t, &mut p, 0x10), Ok(Reading { present: 70, max: 100 }));
    assert_eq!(write(&mut bus, &t, &mut p, 0x10, 40), Ok(()));
    let mon = &m.ddc.mccs.as_ref().unwrap().1;
    assert_eq!(mon.got[0], [0x51, 0x82, 0x01, 0x10, 0xAC]);
    assert_eq!(mon.got[1][..6], [0x51, 0x84, 0x03, 0x10, 0x00, 40]);
    assert_eq!(mon.value(0x10), Some(40));
    let cmds = m.writes.iter().filter(|w| w.0 == GMBUS1 && w.1 & SW_RDY != 0);
    assert!(cmds.filter(|w| w.1 & CYCLE_STOP == 0).all(|w| (w.1 >> 1) & 0x7F == 0x37));
    assert_eq!(m.get(GMBUS0), 0, "pins let go");
}

#[test]
fn the_wrong_pins_or_ddi_a_are_absent_and_the_controller_is_cleared() {
    let mut m = monitor_on(4);
    assert_eq!(ask(&mut m, 1, 0x10), Err(DdcError::Absent));
    assert!(m.writes.contains(&(GMBUS1, SW_CLR_INT)), "interrupt cleared");
    assert!(m.ddc.mccs.as_ref().unwrap().1.got.is_empty(), "nothing reached it");
    assert_eq!(ask(&mut m, 0, 0x10), Err(DdcError::Absent), "DDI A has no DDC pins");
}

#[test]
fn a_spoiled_reply_byte_is_not_believed() {
    let good = crate::model::mccs_reply::get_reply(&[(0x10, 70, 100)], 0x10);
    for (at, original) in good.into_iter().enumerate() {
        let mut m = monitor_on(5);
        m.ddc.mccs.as_mut().unwrap().1.spoil = Some((at, original ^ 0x5A));
        assert!(ask(&mut m, 1, 0x10).is_err(), "byte {at} spoiled and believed");
    }
}
