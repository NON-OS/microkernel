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
//! A board's other devices on the touchpad's bus: a GPIO expander and a
//! thermal sensor, at addresses the sweep never tries.

use nonos_i2cmodel::{Bus, BusEvent, Config, Designware, Expander, HidOverI2c, Sensor, Shared};

use super::client::EXPANDER;
use super::fixture::core;
use crate::regs::{self, Attached};

/// An LM75-type thermal sensor's usual address.
pub const SENSOR: u8 = 0x4C;

pub struct Board {
    pub expander: Shared<Expander>,
    pub sensor: Shared<Sensor>,
    _attached: Attached,
}

/// An expander at EXPANDER and a sensor at SENSOR on one modelled bus,
/// with `pad` beside them if there is one.
pub fn board(pad: Option<HidOverI2c>) -> Board {
    let expander = Shared::new(Expander::new(EXPANDER));
    let sensor = Shared::new(Sensor::new(SENSOR));
    let bus = Bus::default().with(expander.clone()).with(sensor.clone());
    let bus = match pad {
        Some(pad) => bus.with(pad),
        None => bus,
    };
    Board { expander, sensor, _attached: regs::attach(Designware::new(Config::LPSS, bus)) }
}

impl Board {
    /// Neither device was addressed on the wire, read or written.
    pub fn assert_untouched(&self) {
        for (name, at) in [("expander", EXPANDER), ("sensor", SENSOR)] {
            assert!(!addressed(at), "the {name} at {at:#04x} was addressed");
        }
        assert_eq!(self.expander.lock().reads(), 0, "the expander's latch was read");
        assert!(self.expander.lock().writes().is_empty(), "the expander was written");
        assert_eq!(self.sensor.lock().reads(), 0, "the sensor was read");
        assert!(self.sensor.lock().writes().is_empty(), "the sensor was written");
    }
}

/// Whether a START or a repeated START on the wire named `addr`.
fn addressed(addr: u8) -> bool {
    core(|c| {
        c.bus().trace().iter().any(|e| match *e {
            BusEvent::Start { addr: a, .. } | BusEvent::Restart { addr: a, .. } => a == addr,
            _ => false,
        })
    })
}
