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

//! The controller's one shared interrupt, taken apart: each pending pin
//! cleared and delivered to its owner alone, a storming pin masked and
//! reported once.

use crate::controller::Controller;
use crate::router::{Owner, Router};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delivery {
    /// To the holder of this broker device.
    Holder { device: u64, pin: u16 },
    /// To capsule_acpi, for the `_AEI` event method.
    Acpi { pin: u16 },
    /// The pin stormed and is now masked; said once.
    Stormed { pin: u16, owner: Owner },
}

impl Router {
    /// The armed pins not masked.
    fn live(&self) -> Vec<u16> {
        self.bindings.iter().filter(|b| !b.masked).map(|b| b.pin).collect()
    }

    pub fn interrupt(&mut self, ctl: &mut impl Controller, now_ms: u64) -> Vec<Delivery> {
        let pending = ctl.take_pending(&self.live());
        let mut out = Vec::new();
        for pin in pending {
            let Some(b) = self.bindings.iter_mut().find(|b| b.pin == pin && !b.masked) else {
                continue;
            };
            if b.storm.event(now_ms) {
                b.masked = ctl.mask(pin, true).is_ok();
                out.push(Delivery::Stormed { pin, owner: b.owner });
                continue;
            }
            out.push(match b.owner {
                Owner::Device(device) => Delivery::Holder { device, pin },
                Owner::Aei => Delivery::Acpi { pin },
            });
        }
        out
    }
}
