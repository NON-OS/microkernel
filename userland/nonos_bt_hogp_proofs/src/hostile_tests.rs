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

//! Peripherals that would keep the host going round, or point it outside
//! the characteristic it is reading.

use crate::refuse_tests::failed;
use crate::sim::devices::{NOTIFY, READ};
use crate::sim::{keyboard_mouse, Db, Peripheral, Run};
use nonos_bt_hogp::HogpError;

#[test]
fn a_peripheral_that_does_not_move_forward_or_never_ends() {
    let mut peer = Peripheral::new(keyboard_mouse());
    peer.repeat = true;
    failed(&mut Run::with(peer), HogpError::Protocol);
}

#[test]
fn a_descriptor_outside_its_characteristic_is_refused() {
    let db = Db::default()
        .service(0x1812)
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2908, &[0, 1])
        .char(READ | NOTIFY, 0x2a33, &[0; 3])
        .desc(0x2902, &[0, 0]);
    let mut peer = Peripheral::new(db);
    peer.wide = true;
    failed(&mut Run::with(peer), HogpError::Protocol);
}
