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

//! The state word a proof's row carries.

/// VERIFIED and FAILED are verdicts a gate reached. SELF-REPORTED is a pass on
/// the loader's own word, with nothing measuring it. NOT VERIFIED is a record
/// with no pass on it, which only a development boot runs past. NOT CHECKED is
/// a gate that had nothing to check. UNKNOWN is a question the kernel did not
/// answer, and never a pass.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    Verified,
    SelfReported,
    NotVerified,
    NotChecked,
    Failed,
    Unknown,
}

impl Mark {
    pub fn word(self) -> &'static str {
        match self {
            Mark::Verified => "VERIFIED",
            Mark::SelfReported => "SELF-REPORTED",
            Mark::NotVerified => "NOT VERIFIED",
            Mark::NotChecked => "NOT CHECKED",
            Mark::Failed => "FAILED",
            Mark::Unknown => "UNKNOWN",
        }
    }
}
