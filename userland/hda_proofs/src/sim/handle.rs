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

//! The running model as a test holds it: the register window, the rings, and
//! what the driver sent and set.

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use nonos_devmodel::{FakeBar, LiveDevice};

use crate::regs::Regs;

/// Codec coefficients the model holds, by codec, node and index.
pub type Coefs = Mutex<HashMap<(u8, u8, u16), u16>>;

pub struct Sim {
    pub bar: Arc<FakeBar>,
    pub corb: Arc<FakeBar>,
    pub rirb: Arc<FakeBar>,
    pub log: Arc<Mutex<Vec<u32>>>,
    pub plugged: Arc<AtomicBool>,
    pub coefs: Arc<Coefs>,
    pub(super) _dev: LiveDevice,
}

impl Sim {
    pub fn regs(&self) -> Regs {
        Regs::new(self.bar.base())
    }

    pub fn link(&self) -> crate::controller::verb::Link {
        crate::controller::verb::Link::new(self.regs(), self.corb.base(), self.rirb.base(), 256)
    }

    pub fn sent(&self) -> Vec<u32> {
        self.log.lock().unwrap().clone()
    }
}
