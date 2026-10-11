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

//! The meter's one job at a time (plan/21, v1.4.14), its result answered
//! once by JOB, its id never reused within a boot. Pure.

use alloc::vec::Vec;

use nonos_payment_proto::{EAGAIN, EBUSY, ENOENT};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Stage {
    Running,
    Done(Result<Vec<u8>, i32>),
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Jobs {
    last_id: u32,
    current: Option<(u32, Stage)>,
}

impl Jobs {
    /// A new job's id, or EBUSY while one runs. Only the newest result is
    /// kept: one never asked for is let go.
    pub fn start(&mut self) -> Result<u32, i32> {
        if matches!(self.current, Some((_, Stage::Running))) {
            return Err(EBUSY);
        }
        let id = self.last_id.checked_add(1).ok_or(EBUSY)?;
        self.last_id = id;
        self.current = Some((id, Stage::Running));
        Ok(id)
    }

    pub fn running(&self) -> Option<u32> {
        match self.current {
            Some((id, Stage::Running)) => Some(id),
            _ => None,
        }
    }

    /// The running job's result; another id changes nothing.
    pub fn finish(&mut self, id: u32, result: Result<Vec<u8>, i32>) {
        if self.running() == Some(id) {
            self.current = Some((id, Stage::Done(result)));
        }
    }

    /// JOB(id): EAGAIN while it runs, then its result once; ENOENT for any
    /// id that is not the current job's.
    pub fn take(&mut self, id: u32) -> Result<Vec<u8>, i32> {
        match &self.current {
            Some((cur, Stage::Running)) if *cur == id => Err(EAGAIN),
            Some((cur, Stage::Done(_))) if *cur == id => match self.current.take() {
                Some((_, Stage::Done(r))) => r,
                _ => Err(ENOENT),
            },
            _ => Err(ENOENT),
        }
    }
}
