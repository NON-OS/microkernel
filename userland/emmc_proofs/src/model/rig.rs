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

//! A rig: the model host and card, and host memory for the two DMA regions
//! at bus addresses of the test's choosing.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::card::CardCfg;
use super::host::{HostCfg, Sim};
use super::test_env::TestLog;
use crate::emmc::disk::DATA_BUF_BYTES;
use crate::emmc::env::DmaBuf;

pub const TABLE_BUS: u64 = 0x0010_0000;
pub const DATA_BUS: u64 = 0x0020_0000;

pub struct Rig {
    pub sim: Rc<RefCell<Sim>>,
    pub now: Rc<Cell<u64>>,
    pub log: TestLog,
    pub table: DmaBuf,
    pub data: DmaBuf,
    _table_mem: Box<[u8]>,
    _data_mem: Box<[u8]>,
}

impl Rig {
    pub fn new(host: HostCfg, card: CardCfg) -> Self {
        Self::at(host, card, TABLE_BUS, DATA_BUS)
    }

    /// A rig whose DMA regions sit at the given bus addresses.
    pub fn at(host: HostCfg, card: CardCfg, table_bus: u64, data_bus: u64) -> Self {
        let now = Rc::new(Cell::new(0));
        let mut table_mem = vec![0u8; 4096].into_boxed_slice();
        let mut data_mem = vec![0u8; DATA_BUF_BYTES].into_boxed_slice();
        let table =
            DmaBuf { va: table_mem.as_mut_ptr() as u64, bus: table_bus, len: table_mem.len() };
        let data = DmaBuf { va: data_mem.as_mut_ptr() as u64, bus: data_bus, len: data_mem.len() };
        let mut sim = Sim::new(host, card, now.clone());
        sim.mem.push((table_bus, table.va as usize, table.len));
        sim.mem.push((data_bus, data.va as usize, data.len));
        Self {
            sim: Rc::new(RefCell::new(sim)),
            now,
            log: TestLog::default(),
            table,
            data,
            _table_mem: table_mem,
            _data_mem: data_mem,
        }
    }
}
