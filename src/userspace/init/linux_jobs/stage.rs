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

//! The stages an asked-for install passes through, and what an installer's
//! exit makes of one. Apart from the table so the proofs can hold it
//! (`kernel_proofs`).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Stage {
    Queued,
    Running(u32),
    Installed,
    /// The installer's exit code: `install::Why` in the personality.
    Failed(i32),
    /// Init would not start it: the market withdrew it, or the spawn gate refused.
    Refused,
    /// The personality is taking the package away.
    Removing(u32),
    Removed,
}

impl Stage {
    /// Nothing more will happen to it without a new request.
    pub(crate) fn finished(self) -> bool {
        matches!(self, Stage::Installed | Stage::Removed | Stage::Failed(_) | Stage::Refused)
    }

    /// The installer or remover it waits on, while one runs.
    pub(crate) fn pid(self) -> Option<u32> {
        match self {
            Stage::Running(pid) | Stage::Removing(pid) => Some(pid),
            _ => None,
        }
    }

    /// What it becomes once its process ended with `code`. A stage with no
    /// process stays as it is: a queued install has not started yet.
    pub(crate) fn ended(self, code: i32) -> Stage {
        match (self, code) {
            (Stage::Removing(_), 0) => Stage::Removed,
            (Stage::Running(_), 0) => Stage::Installed,
            (Stage::Running(_) | Stage::Removing(_), code) => Stage::Failed(code),
            (other, _) => other,
        }
    }
}
