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

//! The capsule's side of `SleepHardware`: whether it holds the Global
//! Lock, which it does only inside an evaluation (`host/lock_hold.rs`), so
//! the value between evaluations, read when the sequence starts, is the
//! value at the moment of sleep, when none runs. Everything else is the
//! kernel's, with no kernel path yet: each answers NotWired, and S3 is
//! undone, not entered.

use nonos_aml::sleep::SleepTypes;
use nonos_aml::suspend::{HwError, SleepHardware};

pub struct Unwired {
    pub lock_held: bool,
}

impl SleepHardware for Unwired {
    fn global_lock_held(&self) -> bool {
        self.lock_held
    }
    fn set_waking_vector(&mut self, _: u64) -> Result<(), HwError> {
        Err(HwError::NotWired)
    }
    fn clear_wake_status(&mut self) -> Result<(), HwError> {
        Err(HwError::NotWired)
    }
    fn arm_gpe(&mut self, _: u16, _: bool) -> Result<(), HwError> {
        Err(HwError::NotWired)
    }
    fn sleep(&mut self, _: SleepTypes) -> Result<(), HwError> {
        Err(HwError::NotWired)
    }
}
