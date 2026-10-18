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

//! A model of VirtualBox's VMMDev as its device code answers: requests
//! carried out within the write of their address to the request port, the
//! event page and its pending flag, and the host's pointer with its mouse
//! integration switch. Shared between the shim's port hook and the test.

mod carry;
mod host;
mod pointer;
mod requests;

pub use host::{Host, Options};

use std::cell::RefCell;
use std::rc::Rc;

use nonos_libc::{install_ports, set_memory, Ports};

pub type Shared = Rc<RefCell<Host>>;

struct Hook(Shared);

impl Ports for Hook {
    fn write(&mut self, grant: u64, offset: u16, width: u8, value: u32) {
        assert_eq!((grant, offset, width), (50, 0, 4), "a doubleword to the request port");
        self.0.borrow_mut().request(value);
    }
}

/// A guest device on this thread's ports, as the options make it.
pub fn install(opt: Options) -> Shared {
    let host = Rc::new(RefCell::new(Host::new(opt)));
    set_memory(host.borrow().memory_va());
    install_ports(Box::new(Hook(host.clone())));
    host
}
