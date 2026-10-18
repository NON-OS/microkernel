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

//! A device that answers a recovery's own commands and stalls on the next
//! present, every time, was reset and recovered forever, 2 s or more a
//! cycle, reporting Live. After IN_A_ROW recoveries without serving
//! SERVED_MS after one the next stall leaves it `Down`. A request it never
//! sees is not serving, however long it has been since the recovery; once
//! down, nothing is written to its rings, its doorbell is not rung and
//! nothing waits on it.

use nonos_libc::{releases, said};

use super::client::{call, flush};
use super::model::NOTIFY_AT;
use super::part::{answering_after_reset, ring::bytes, Answer};
use super::stall::{handle, health, stalled, whole_flush, CLIENT, PRIMARY};
use crate::clock::{now_ms, pass};
use crate::constants::VQ_REGION_SIZE;
use crate::driver::Driver;
use crate::protocol::{E_DEVICE, E_INVAL, OP_FLUSH};
use crate::server::recover::policy::{IN_A_ROW, SERVED_MS};
use crate::server::recover::settle;
use crate::state::Health;

/// A recovery's own commands: create, attach, transfer, scanout, flush.
pub const RECOVERY: u16 = 5;

/// A flush of the primary, then what the server does after every request.
pub fn present(d: &mut Driver, body: &[u8]) -> i32 {
    let status = call(d, CLIENT, OP_FLUSH, body);
    settle(d);
    status
}

#[test]
fn a_device_that_stalls_on_every_present_is_down_after_in_a_row_recoveries() {
    let (mut d, region, regs) = stalled();
    let _part = answering_after_reset(&region, &regs, Answer::Spec, RECOVERY);
    settle(&mut d);
    let off_the_surface = flush(PRIMARY.resource_id, PRIMARY.width, 0, 1, 1);
    for _ in 0..IN_A_ROW {
        pass(SERVED_MS);
        assert_eq!(present(&mut d, &off_the_surface), E_INVAL, "refused before the device");
        assert_eq!(present(&mut d, &whole_flush()), E_DEVICE, "the present stalled");
    }
    assert_eq!(d.health, Health::Down, "recovered again after {IN_A_ROW} in a row");
    assert_eq!(health(&d), (1, IN_A_ROW), "no recovery past the count");
    assert!(said().last().is_some_and(|s| s.contains("down") && s.contains("in a row")));
    assert!(d.control_queue.is_reset(), "the stalled queue was left in service");
    let bell = NOTIFY_AT + d.control_queue.regs.doorbell;
    regs.present16(bell, 0xFFFF);
    let (rings, from) = (bytes(&region, 0, VQ_REGION_SIZE), now_ms());
    let one_pixel = flush(PRIMARY.resource_id, 0, 0, 1, 1);
    assert_eq!(present(&mut d, &one_pixel), E_DEVICE);
    assert!(bytes(&region, 0, VQ_REGION_SIZE) == rings, "written to the rings once down");
    assert_eq!(regs.wrote16(bell), 0xFFFF, "the doorbell rung once it was down");
    assert_eq!(now_ms(), from, "a wait on the device once it was down");
    assert_eq!(handle(&d), Some(PRIMARY.handle), "the surface is still served");
    assert_eq!(releases(), 0, "the claim and every grant are kept");
}
