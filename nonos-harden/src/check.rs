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

//! One check's result. A check that could not run is not a pass: GO needs
//! every check to have run and passed.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Pass,
    Fail,
    NotRun,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Check {
    pub id: &'static str,
    pub status: Status,
    pub reason: String,
    /// What this check, though it passed, says it does not cover.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub limits: Vec<String>,
}

impl Check {
    pub fn pass(id: &'static str, reason: impl Into<String>) -> Self {
        Self::new(id, Status::Pass, reason)
    }
    pub fn fail(id: &'static str, reason: impl Into<String>) -> Self {
        Self::new(id, Status::Fail, reason)
    }
    pub fn not_run(id: &'static str, reason: impl Into<String>) -> Self {
        Self::new(id, Status::NotRun, reason)
    }
    pub fn with_limits(mut self, limits: Vec<String>) -> Self {
        self.limits = limits;
        self
    }
    fn new(id: &'static str, status: Status, reason: impl Into<String>) -> Self {
        Self { id, status, reason: reason.into(), limits: Vec::new() }
    }
}

/// Reasons name at most this many problems, then say how many more.
pub const NAMED: usize = 8;

/// The first problems by name, then a count of the rest.
pub fn named(problems: Vec<String>) -> String {
    let more = problems.len().saturating_sub(NAMED);
    let mut s = problems.into_iter().take(NAMED).collect::<Vec<_>>().join("; ");
    if more > 0 {
        s += &format!("; and {more} more");
    }
    s
}
