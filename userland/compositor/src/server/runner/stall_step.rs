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

//! The serve loop's steps, and what the breadcrumbs say of it at a look.

/// What the loop is doing, as the breadcrumbs record it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Step {
    Wait = 0,
    Request = 1,
    Upkeep = 2,
    Compose = 3,
    Flush = 4,
}

impl From<u8> for Step {
    fn from(v: u8) -> Step {
        [Step::Wait, Step::Request, Step::Upkeep, Step::Compose, Step::Flush]
            .get(v as usize)
            .copied()
            .unwrap_or(Step::Wait)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seen {
    pub turn: u32,
    pub step: Step,
    pub since: i64,
    pub op: u16,
    pub from: u32,
}
