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

//! The model a driver run is held against, and the rig that wires the real
//! driver to it: host memory standing in for DMA regions, a clock that
//! moves one millisecond per read, and a log the tests read back.

pub mod card;
pub mod host;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub use card::{CardCfg, St};
pub use host::{CmdRec, HostCfg, Sim, SimBus, GLK_CAPS};

use crate::emmc::disk::{EmmcDisk, DATA_BUF_BYTES};
use crate::emmc::env::{Clock, DmaBuf, Log};
use crate::emmc::error::EmmcResult;
use crate::emmc::sdhci::Host;

/// Each read moves time on by one millisecond, so every bounded wait in the
/// driver ends, and its bound is measured in reads.
#[derive(Clone)]
pub struct TestClock(pub Rc<Cell<u64>>);

impl Clock for TestClock {
    fn now_ms(&self) -> u64 {
        let t = self.0.get() + 1;
        self.0.set(t);
        t
    }
}

#[derive(Clone, Default)]
pub struct TestLog(pub Rc<RefCell<Vec<String>>>);

impl Log for TestLog {
    fn line(&self, text: &[u8]) {
        self.0.borrow_mut().push(String::from_utf8_lossy(text).into_owned());
    }
}

pub type TestHost = Host<SimBus, TestClock, TestLog>;
pub type TestDisk = EmmcDisk<SimBus, TestClock, TestLog>;

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

    pub fn host(&self, intel_emmc: bool) -> TestHost {
        Host::new(
            SimBus(self.sim.clone()),
            TestClock(self.now.clone()),
            self.log.clone(),
            self.table,
            intel_emmc,
        )
    }

    pub fn disk(&self, intel_emmc: bool) -> EmmcResult<TestDisk> {
        EmmcDisk::bring_up(self.host(intel_emmc), self.data)
    }

    pub fn violations(&self) -> Vec<String> {
        self.sim.borrow().violations.clone()
    }

    pub fn cmds(&self) -> Vec<CmdRec> {
        self.sim.borrow().cmds.clone()
    }

    pub fn indices(&self) -> Vec<u8> {
        self.cmds().iter().map(|c| c.index).collect()
    }

    pub fn logs(&self) -> Vec<String> {
        self.log.0.borrow().clone()
    }

    pub fn logged(&self, needle: &str) -> bool {
        self.logs().iter().any(|l| l.contains(needle))
    }

    pub fn ms(&self) -> u64 {
        self.now.get()
    }

    /// Fail the test with the model's complaints and the driver's log.
    pub fn assert_clean(&self) {
        let v = self.violations();
        assert!(v.is_empty(), "model violations: {v:#?}\nlog: {:#?}", self.logs());
    }
}
