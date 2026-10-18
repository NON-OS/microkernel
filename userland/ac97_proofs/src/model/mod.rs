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

//! A model of the 82801AA AC'97 controller and its codec, answering each
//! port access the way the ICH data sheet, AC'97 2.3 and QEMU's ac97 do,
//! and playing what the engine fetches through the descriptor list. The
//! model is shared between the shim's port hook and the test.

mod codec;
mod engine;
mod ich;
mod nabm;
mod nabm_write;
mod nam;
mod pcm_box;

pub use codec::{EXT_CTRL, FRONT_RATE, HEADPHONE, MASTER, PCM_OUT};
pub use ich::{Ich, Options};

use std::cell::RefCell;
use std::rc::Rc;

use nonos_libc::{install_ports, Ports};

/// Grant ids the shim hands out: 50 plus the BAR.
pub const NAM_GRANT: u64 = 50;
pub const NABM_GRANT: u64 = 51;

pub type Shared = Rc<RefCell<Ich>>;

struct Hook(Shared);

impl Ports for Hook {
    fn read(&mut self, grant: u64, offset: u16, width: u8) -> u32 {
        let mut ich = self.0.borrow_mut();
        match grant {
            NAM_GRANT => ich.nam_read(offset, width),
            NABM_GRANT => ich.nabm_read(offset, width),
            _ => panic!("read through unknown grant {grant}"),
        }
    }

    fn write(&mut self, grant: u64, offset: u16, width: u8, value: u32) {
        let mut ich = self.0.borrow_mut();
        match grant {
            NAM_GRANT => ich.nam_write(offset, width, value),
            NABM_GRANT => ich.nabm_write(offset, width, value),
            _ => panic!("write through unknown grant {grant}"),
        }
    }
}

/// A controller on this thread's ports, as the options make it.
pub fn install(opt: Options) -> Shared {
    let ich = Rc::new(RefCell::new(Ich::new(opt)));
    install_ports(Box::new(Hook(ich.clone())));
    ich
}
