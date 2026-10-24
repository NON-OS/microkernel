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

//! The modelled host and device as the engine's `Hw`: registers, a clock
//! that moves a millisecond each time it is read, and the console lines.

use std::cell::{Cell, RefCell};

use super::cfg::Cfg;
use super::state::{Dev, Host};
use crate::ufs::env::{Clock, Log};

pub struct Model {
    pub cfg: RefCell<Cfg>,
    pub host: RefCell<Host>,
    pub dev: RefCell<Dev>,
    pub now: Cell<u64>,
    pub lines: RefCell<Vec<String>>,
    /// A device that lies: in its responses of this type, this byte.
    pub lie: Cell<Option<(u8, usize, u8)>>,
    /// A device that answers GOOD but sends no data for this opcode.
    pub mute: Cell<Option<u8>>,
}

impl Model {
    pub fn new(cfg: Cfg) -> Self {
        let host = Host { lcc: cfg.lcc, regs: vec![0; 64], ..Host::default() };
        let m = Self {
            cfg: RefCell::new(cfg),
            host: RefCell::new(host),
            dev: RefCell::new(Dev::default()),
            now: Cell::new(0),
            lie: Cell::new(None),
            mute: Cell::new(None),
            lines: RefCell::new(Vec::new()),
        };
        let (cap, ver) = (m.cfg.borrow().cap, m.cfg.borrow().ver);
        m.host.borrow_mut().regs[0] = cap;
        m.host.borrow_mut().regs[2] = ver;
        m
    }

    pub fn said(&self, text: &str) -> bool {
        self.lines.borrow().iter().any(|l| l.contains(text))
    }
}

impl Clock for Model {
    fn now_ms(&self) -> u64 {
        let t = self.now.get();
        self.now.set(t + 1);
        t
    }
}

impl Log for Model {
    fn line(&self, text: &[u8]) {
        self.lines.borrow_mut().push(String::from_utf8_lossy(text).into_owned());
    }
}
