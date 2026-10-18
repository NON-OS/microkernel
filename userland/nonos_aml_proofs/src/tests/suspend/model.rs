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

//! A model platform for suspend: one timeline of every method the sequence
//! evaluates (its path and integer arguments) and every hardware write
//! (waking vector, WAK_STS, GPE enables, the PM1 sleep write), with the
//! Global Lock's state and any step the model refuses set by the test.

use crate::sleep::SleepTypes;
use crate::suspend::{HwError, SleepHardware};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

#[derive(Default)]
pub struct Model {
    pub log: Log,
    pub lock_held: bool,
    /// The step that answers NotWired: "vector", "clear" or "sleep".
    pub unwired: Option<&'static str>,
    pub armed: BTreeSet<u16>,
    /// The GPEs armed at the moment of the sleep write.
    pub armed_asleep: Option<BTreeSet<u16>>,
}

impl Model {
    fn step(&mut self, name: &'static str, entry: String) -> Result<(), HwError> {
        if self.unwired == Some(name) {
            return Err(HwError::NotWired);
        }
        self.log.borrow_mut().push(entry);
        Ok(())
    }
}

impl SleepHardware for Model {
    fn global_lock_held(&self) -> bool {
        self.lock_held
    }
    fn set_waking_vector(&mut self, address: u64) -> Result<(), HwError> {
        self.step("vector", format!("hw vector {address:#x}"))
    }
    fn clear_wake_status(&mut self) -> Result<(), HwError> {
        self.step("clear", "hw clear WAK_STS".into())
    }
    fn arm_gpe(&mut self, gpe: u16, on: bool) -> Result<(), HwError> {
        if on {
            self.armed.insert(gpe)
        } else {
            self.armed.remove(&gpe)
        };
        self.step("gpe", format!("hw gpe {gpe:#x} {}", if on { "on" } else { "off" }))
    }
    fn sleep(&mut self, t: SleepTypes) -> Result<(), HwError> {
        self.armed_asleep = Some(self.armed.clone());
        self.step("sleep", format!("hw sleep {}/{}", t.a, t.b))
    }
}
