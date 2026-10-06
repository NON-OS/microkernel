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

//! The model on the scripted bus: GET_DESCRIPTOR from the descriptors,
//! the vendor register requests (0xC0/0x40, request 0x05) from and into
//! the register space, any other request accepted. A test may refuse one
//! request, by bRequest and wValue, with an errno.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_usbnet::mock::{descriptors, Call, MockBus};

use super::descriptors::{device, CONFIG_COMM, CONFIG_VENDOR};
use super::power::{power_on, settle};
use super::regs::Regs;

pub struct Chip {
    pub regs: Regs,
    pub refuse: Option<(u8, u16, i32)>,
}

pub type Shared = Rc<RefCell<Chip>>;

/// A chip of PLA_TCR1 `version` behind the adapter IDs `ids`.
pub fn rtl8153(version: u16, ids: (u16, u16)) -> (MockBus, Shared) {
    let chip = Rc::new(RefCell::new(Chip { regs: power_on(version), refuse: None }));
    let model = chip.clone();
    let bus = MockBus::new(Box::new(move |call| {
        let configs: [&[u8]; 2] = [&CONFIG_COMM, &CONFIG_VENDOR];
        if let Some(bytes) = descriptors(call, &device(ids), &configs, &[]) {
            return Ok(bytes);
        }
        let mut c = model.borrow_mut();
        let setup = match call {
            Call::In(s, _) | Call::Out(s, _) => *s,
            _ => return Ok(vec![]),
        };
        if let Some((request, value, e)) = c.refuse {
            if setup.request == request && setup.value == value {
                return Err(e);
            }
        }
        let ty = setup.index & 0xff00;
        match call {
            Call::In(s, len) if s.request_type == 0xc0 && s.request == 0x05 => {
                Ok(c.regs.read(ty, s.value, *len))
            }
            Call::Out(s, data) if s.request_type == 0x40 && s.request == 0x05 => {
                c.regs.write(ty, s.value, s.index as u8, data);
                settle(&mut c.regs);
                Ok(vec![])
            }
            _ => Ok(vec![]),
        }
    }));
    (bus, chip)
}
