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

//! What an S3 attempt did: whether the machine slept, what stopped it if
//! not, \_S0's sleep types for the way back, the devices armed and those
//! passed over, and what \_WAK answered.

use super::hardware::HwError;
use super::wake::Skip;
use crate::exec::Fault;
use crate::namespace::Id;
use crate::object::Object;
use crate::sleep::SleepTypes;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuspendError {
    /// \_S3 is absent or unusable: nothing was run.
    NoS3,
    /// OSPM held the Global Lock at the moment of sleeping.
    LockHeld,
    Hardware(HwError),
}

#[derive(Debug, Default)]
pub struct Outcome {
    pub entered: bool,
    pub error: Option<SuspendError>,
    pub s0: Option<SleepTypes>,
    pub armed: Vec<Id>,
    pub skipped: Vec<(Id, Skip)>,
    /// None when \_WAK is absent, or nothing ran.
    pub wak: Option<Result<Object, Fault>>,
}

impl Outcome {
    pub fn refused(e: SuspendError) -> Self {
        Outcome { error: Some(e), ..Outcome::default() }
    }
}
