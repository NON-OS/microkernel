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

//! Checking a session record offline (nsh-v1-record.md, "Checking a record
//! offline"), and its checkpoints, `nsh-ckpt/1`. A record with a footer is
//! complete only if every step passes; one without is checked as far as
//! its last checkpoint and reported as cut short there, never as complete.

pub(crate) mod bound;
mod ckpt;
mod cut;
mod header;
mod partial;
mod report;
mod verdict;
mod verify;
mod walk;

pub use bound::{Bound, Mismatch};
pub use ckpt::CKPT_LEN;
pub use report::{Registry, Report};
pub use verdict::{CkptRefusal, RecordRefusal, Verdict};
pub use verify::verify_record;
