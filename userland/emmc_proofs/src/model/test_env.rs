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

//! The clock and log a rig's driver runs on, and the types it is built of.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::host::SimBus;
use crate::emmc::disk::EmmcDisk;
use crate::emmc::env::{Clock, Log};
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
