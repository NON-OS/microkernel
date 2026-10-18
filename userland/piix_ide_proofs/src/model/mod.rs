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

//! A model of the PIIX IDE function: two channels with their drives, the
//! bus master window over both, answering each port access the way ATA, the
//! 82371AB data sheet and QEMU's piix do. The model is shared between the
//! shim's port hook and the test.

mod channel;
mod drive;

pub use channel::{table, Chan, Pending};
pub use drive::{put_text, Drive, Fault, Kind};

use std::cell::RefCell;
use std::rc::Rc;

use nonos_libc::{install_ports, Ports};

/// Grant ids the shim hands out: 50 plus the BAR.
pub const CMD0: u64 = 50;
pub const CTL0: u64 = 51;
pub const CMD1: u64 = 52;
pub const CTL1: u64 = 53;
pub const BM: u64 = 54;

pub struct Piix {
    pub ch: [Chan; 2],
    /// Where the control register sits in a control window: 0 for the
    /// compatibility-mode register alone, 2 in a native four-port BAR.
    pub ctl_off: u16,
}

pub type Shared = Rc<RefCell<Piix>>;

struct Hook(Shared);

impl Ports for Hook {
    fn read(&mut self, grant: u64, offset: u16, width: u8) -> u32 {
        let mut p = self.0.borrow_mut();
        let ctl_off = p.ctl_off;
        match grant {
            CMD0 | CMD1 => p.ch[((grant - CMD0) / 2) as usize].cmd_read(offset, width),
            CTL0 | CTL1 => {
                assert_eq!((offset, width), (ctl_off, 1), "the control register");
                p.ch[((grant - CTL0) / 2) as usize].status().into()
            }
            BM => p.ch[(offset / 8) as usize].bm_read(offset % 8, width),
            _ => panic!("read through unknown grant {grant}"),
        }
    }

    fn write(&mut self, grant: u64, offset: u16, width: u8, value: u32) {
        let mut p = self.0.borrow_mut();
        let ctl_off = p.ctl_off;
        match grant {
            CMD0 | CMD1 => {
                assert_eq!(width, 1, "task file registers are written a byte at a time");
                p.ch[((grant - CMD0) / 2) as usize].cmd_write(offset, value as u8)
            }
            CTL0 | CTL1 => {
                assert_eq!((offset, width), (ctl_off, 1), "the control register");
                p.ch[((grant - CTL0) / 2) as usize].control(value as u8)
            }
            BM => p.ch[(offset / 8) as usize].bm_write(offset % 8, width, value),
            _ => panic!("write through unknown grant {grant}"),
        }
    }
}

/// A controller on this thread's ports, its slots filled as given.
pub fn install(slots: [[Option<Drive>; 2]; 2], ctl_off: u16) -> Shared {
    let [a, b] = slots;
    let mk = |drives| Chan { drives, ..Chan::default() };
    let p = Rc::new(RefCell::new(Piix { ch: [mk(a), mk(b)], ctl_off }));
    install_ports(Box::new(Hook(p.clone())));
    p
}
