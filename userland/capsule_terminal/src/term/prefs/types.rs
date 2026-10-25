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

/// Saved project slots the record carries. Fixed so `Prefs` stays `Copy` and
/// the on-disk record stays a constant length.
pub const MAX_PROJECTS: usize = 8;
pub use super::project::{Project, PATH_CAP};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Prefs {
    pub theme: u16,
    pub font_scale: u8,
    pub cursor: u8,
    /// Rail flags (`rail_flags.rs`). Bit 1 is whether the left rail is on
    /// screen at all, which is off by default; bit 0 is kept but unused.
    pub rails: u8,
    pub projects: [Project; MAX_PROJECTS],
    pub project_count: u8,
}

impl Prefs {
    pub fn project_slice(&self) -> &[Project] {
        &self.projects[..(self.project_count as usize).min(MAX_PROJECTS)]
    }

    pub fn push_project(&mut self, path: &[u8]) -> bool {
        let n = (self.project_count as usize).min(MAX_PROJECTS);
        if n == MAX_PROJECTS || path.is_empty() {
            return false;
        }
        let slot = Project::new(path);
        if self.projects[..n].iter().any(|p| p.as_bytes() == slot.as_bytes()) {
            return false;
        }
        self.projects[n] = slot;
        self.project_count = (n + 1) as u8;
        true
    }
}

impl Default for Prefs {
    fn default() -> Self {
        Self {
            theme: 0,
            font_scale: 2,
            cursor: 0,
            rails: 0,
            projects: [Project::default(); MAX_PROJECTS],
            project_count: 0,
        }
    }
}
