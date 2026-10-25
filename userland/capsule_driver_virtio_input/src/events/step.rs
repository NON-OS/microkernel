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

//! What one event asks of the driver once the report has taken it in.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Gathered into the report; nothing to do yet.
    Gathered,
    /// The report is complete: post what it gathered.
    Flush,
    /// A keyboard key: Linux key code, and 0 release, 1 press, 2 repeat.
    Key(u16, i32),
    /// Neither a key nor anything the report keeps.
    Ignored,
}
