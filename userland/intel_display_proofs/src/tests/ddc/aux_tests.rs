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

//! DDC/CI over AUX: the request goes in one I2C write to 0x37 with
//! middle-of-transaction set and is ended by an address-only write, the
//! reply is read and ended by an address-only read, every request names
//! 0x37; a sink with nothing at that address (the model's DP monitor)
//! is absent.

use super::caps_rig::known;
use crate::ddc::error::DdcError;
use crate::ddc::reply::Reading;
use crate::ddc::transact::read;
use crate::ddc::transact_write::write;
use crate::dp::i2c_ddc::AuxDdc;
use crate::model::mccs::Mccs;
use crate::model::mccs_aux::MccsAux;
use crate::model::sink::sink;
use crate::model::sink_aux::AuxEnd;
use crate::model::Ticks;

#[test]
fn a_read_and_a_set_are_framed_as_i2c_over_aux_at_0x37() {
    let mut a = MccsAux { monitor: Mccs::with(&[(0x12, 50, 100)]), ..Default::default() };
    let (t, mut p) = (Ticks::new(), known());
    let mut bus = AuxDdc { aux: &mut a };
    assert_eq!(read(&mut bus, &t, &mut p, 0x12), Ok(Reading { present: 50, max: 100 }));
    assert_eq!(write(&mut bus, &t, &mut p, 0x12, 75), Ok(()));
    assert_eq!(a.monitor.value(0x12), Some(75));
    let r = &a.requests;
    assert_eq!(
        r[0],
        [0x40, 0x00, 0x37, 4, 0x51, 0x82, 0x01, 0x12, 0x6E ^ 0x51 ^ 0x82 ^ 0x01 ^ 0x12]
    );
    assert_eq!(r[1], [0x00, 0x00, 0x37], "the write's stop");
    assert_eq!(r[2], [0x50, 0x00, 0x37, 10], "eleven bytes, middle of transaction");
    assert_eq!(r[3], [0x10, 0x00, 0x37], "the read's stop");
    assert!(r.iter().all(|q| q[1..3] == [0x00, 0x37]), "no other address");
}

#[test]
fn a_sink_with_nothing_at_0x37_is_absent() {
    let (t, mut p) = (Ticks::new(), known());
    let mut end = AuxEnd(sink(Vec::new()));
    assert_eq!(read(&mut AuxDdc { aux: &mut end }, &t, &mut p, 0x10), Err(DdcError::Absent));
}
