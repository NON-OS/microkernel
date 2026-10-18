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

//! A controller brought back inside the very request that binds one of the
//! multiplexer's slot ids to it. The slot is enabled on the revived
//! controller, so the binding is current: dropped as stale by the next
//! request, the id the reply named was refused from then on.

use super::bench::Bench;
use super::hc_device::lock;
use super::hc_reset::Resets;
use crate::conformance::model::CAP_LEN;
use crate::constants::{USBSTS, USBSTS_HSE};
use crate::protocol::{E_INVAL, OP_ADDRESS_DEVICE, OP_DISABLE_SLOT, OP_GET_DEVICE_DESCRIPTOR};

const HIGH_SPEED: u32 = 3;
const STS: usize = CAP_LEN + USBSTS as usize;

#[test]
fn a_slot_bound_in_the_request_that_revived_its_controller_stays_bound() {
    let mut b = Bench::new(HIGH_SPEED);
    let _second = b.second_controller();
    lock(&b.hc).resets = Resets::Followed;
    let v = b.enable_slot();
    lock(&b.hc).fault = true;
    while b.bar.wrote32(STS) & USBSTS_HSE == 0 {
        std::thread::yield_now();
    }
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[v, 1]).status, 0, "addressed once revived");
    assert_eq!(b.mux.ctxs[0].driver.generation, 1, "revived once");
    let desc = b.call(OP_GET_DEVICE_DESCRIPTOR, &[v]);
    assert_eq!(desc.status, 0, "the slot the reply named is still the client's");
    assert_eq!(b.call(OP_DISABLE_SLOT, &[v]).status, 0, "and is given back");
    assert_eq!(b.call(OP_GET_DEVICE_DESCRIPTOR, &[v]).status, E_INVAL, "once");
}
