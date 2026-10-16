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

//! What goes nowhere near the bus: a monitor without DDC/CI is asked once
//! a second however often it is asked; a read-only control, a control the
//! allowlist does not name, and any read without a clock send nothing.

use super::fake_bus::FakeBus;
use crate::ddc::error::DdcError;
use crate::ddc::pace::Pacer;
use crate::ddc::transact::{read, write};
use crate::model::Ticks;

const MONITOR: [(u8, u16, u16); 2] = [(0x10, 70, 100), (0x12, 50, 100)];

#[test]
fn a_refusal_is_kept_too_and_nothing_goes_unpaced_or_unallowed() {
    let (t, mut p) = (Ticks::new(), Pacer::new());
    let mut b = FakeBus::new(&t, &MONITOR);
    b.absent = true;
    for _ in 0..10 {
        assert_eq!(read(&mut b, &t, &mut p, 0x10), Err(DdcError::Absent));
    }
    assert_eq!(b.sends().len(), 1, "a monitor without DDC/CI is asked once a second");
    b.absent = false;
    assert_eq!(write(&mut b, &t, &mut p, 0x10, 101), Err(DdcError::Absent), "max unknown");
    let before = b.log.len();
    assert_eq!(write(&mut b, &t, &mut p, 0x60, 3), Err(DdcError::ReadOnly));
    assert_eq!(read(&mut b, &t, &mut p, 0x14), Err(DdcError::NotAllowed));
    assert_eq!(write(&mut b, &t, &mut p, 0x04, 1), Err(DdcError::NotAllowed), "factory reset");
    let broken = Ticks { broken: true, ..Ticks::new() };
    assert_eq!(read(&mut b, &broken, &mut Pacer::new(), 0x12), Err(DdcError::NoClock));
    assert_eq!(b.log.len(), before, "none of them touched the bus");
}
