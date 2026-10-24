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
//! The caller's side of the walk: a `Bind` device is bound, a hub is
//! brought up with the library's own `attach`, anything else is left.

use nonos_usb_hub::{attach, Classes, Hub, Place};

use super::model::Kind;
use super::state::Fake;

/// A hub's configuration: one interface of class 09h and its interrupt IN
/// status endpoint 81h, 1-byte packets, interval 12.
pub const HUB_CONFIG: [u8; 25] =
    [9, 2, 25, 0, 1, 1, 0, 0xE0, 0, 9, 4, 0, 0, 1, 9, 0, 0, 0, 7, 5, 0x81, 3, 1, 0, 12];

#[derive(Default)]
pub struct Bind {
    pub bound: Vec<u8>,
    pub forgotten: Vec<u8>,
}

impl Classes<Fake> for Bind {
    fn classify(&mut self, h: &mut Fake, slot: u8, place: Place, hubs: &mut Vec<Hub>) -> bool {
        match h.devices.get(&slot).map(|d| d.kind.clone()) {
            Some(Kind::Bind) => {
                self.bound.push(slot);
                true
            }
            Some(Kind::Hub(model)) => {
                h.hubs.insert(slot, model);
                if let Ok(hub) = attach(h, slot, place, &HUB_CONFIG) {
                    hubs.push(hub);
                }
                false
            }
            _ => false,
        }
    }

    fn forget(&mut self, slot: u8) {
        self.forgotten.push(slot);
    }
}

/// `model` as a hub in slot 1 on root port `root` at `speed`, attached.
pub fn root_hub(h: &mut Fake, model: super::HubModel, root: u8, speed: u8) -> Hub {
    h.next_slot = 1;
    h.hubs.insert(1, model);
    attach(h, 1, Place::root(root, speed), &HUB_CONFIG).expect("hub attached")
}
