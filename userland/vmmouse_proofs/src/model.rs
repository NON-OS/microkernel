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

//! QEMU's vmmouse behind its vmport (hw/i386/vmmouse.c, hw/i386/vmport.c):
//! the version call, the pointer's command, status and data calls with
//! QEMU's own checks, and the host's mouse feeding the queue while the
//! guest has asked for absolute positions.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use nonos_libc::set_host;

const MAGIC: u32 = 0x564D_5868;
const VERSION_ID: u32 = 0x3442_554A;
const QUEUE_SIZE: usize = 1024;

pub const VMWARE: u8 = 3;

pub struct Vmmouse {
    /// GETVERSION's EAX.
    pub version: u32,
    /// Off: a vmport with no vmmouse behind it, which leaves every pointer
    /// call's registers as the guest loaded them.
    pub pointer: bool,
    /// The pointer's error half: 0xFFFF while disabled or after a bad call.
    pub status: u16,
    pub queue: VecDeque<u32>,
    pub absolute: bool,
    /// Knobs.
    pub wrong_id: bool,
    /// ENABLE clears the error and queues nothing: the pointer will not
    /// come back.
    pub dead: bool,
    /// ENABLE queues the identity and leaves the error up.
    pub enable_keeps_error: bool,
}

#[derive(Clone)]
pub struct Host(pub Rc<RefCell<Vmmouse>>);

impl Host {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Vmmouse {
            version: 6,
            pointer: true,
            status: 0xFFFF,
            queue: VecDeque::new(),
            absolute: false,
            wrong_id: false,
            dead: false,
            enable_keeps_error: false,
        })))
    }

    /// Behind the backdoor of a host CPUID names by `vendor`.
    pub fn install(&self, vendor: u8) {
        let me = self.0.clone();
        set_host(vendor, Box::new(move |regs| me.borrow_mut().call(regs)));
    }

    /// The host's mouse, in QEMU's absolute range (0 to 0x7FFF), its
    /// buttons (left 1, right 2, middle 4) and wheel (-1 a notch up).
    pub fn mouse(&self, x: u32, y: u32, buttons: u32, dz: i32) {
        let mut s = self.0.borrow_mut();
        if s.status != 0 || !s.absolute || s.queue.len() > QUEUE_SIZE - 4 {
            return;
        }
        let mut b = 0;
        if buttons & 1 != 0 {
            b |= 0x20;
        }
        if buttons & 2 != 0 {
            b |= 0x10;
        }
        if buttons & 4 != 0 {
            b |= 0x08;
        }
        s.queue.extend([b, x << 1, y << 1, dz as u32]);
    }

    /// A packet as the host queues it, word for word.
    pub fn raw(&self, words: &[u32]) {
        self.0.borrow_mut().queue.extend(words);
    }
}

impl Default for Host {
    fn default() -> Self {
        Self::new()
    }
}

impl Vmmouse {
    fn call(&mut self, mut regs: [u32; 4]) -> [u32; 4] {
        let (cmd, arg) = (regs[2], regs[1]);
        match cmd {
            10 => {
                regs[0] = self.version;
                regs[1] = MAGIC;
            }
            _ if !self.pointer => {}
            41 => match arg {
                0x4541_4552 => {
                    if self.dead {
                        self.status = 0;
                    } else if self.queue.len() < QUEUE_SIZE {
                        self.queue.push_back(if self.wrong_id { 0x1234 } else { VERSION_ID });
                        if !self.enable_keeps_error {
                            self.status = 0;
                        }
                    }
                }
                0xF5 => {
                    self.status = 0xFFFF;
                    self.queue.clear();
                    self.absolute = false;
                }
                0x5342_4152 => self.absolute = true,
                0x4C45_5252 => self.absolute = false,
                _ => panic!("pointer command {arg:#x}"),
            },
            40 => regs[0] = (self.status as u32) << 16 | self.queue.len() as u32,
            39 => {
                let n = arg as usize;
                if n == 0 || n > 6 || n > self.queue.len() {
                    self.status = 0xFFFF;
                    return regs;
                }
                for r in regs.iter_mut().take(n.min(4)) {
                    *r = self.queue.pop_front().unwrap();
                }
                self.queue.drain(..n.saturating_sub(4));
            }
            _ => panic!("backdoor command {cmd}"),
        }
        regs
    }
}
