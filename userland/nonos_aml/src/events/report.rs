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

//! What one SCI did, for the capsule's log.

use crate::exec::Fault;
use alloc::vec::Vec;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SciReport {
    /// Fixed events served and cleared.
    pub fixed: u16,
    /// Runtime GPEs run, in order, and how their methods ended.
    pub ran: Vec<(u16, Result<(), Fault>)>,
    /// GPEs left disabled: for good (None) after a level method failed on
    /// its own, or for a back-off window in milliseconds after cuts.
    pub disabled: Vec<(u16, Option<u64>)>,
    /// GPEs turned off for firing past the storm bound (`storm`).
    pub stormed: Vec<u16>,
}
