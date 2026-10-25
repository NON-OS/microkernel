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

//! What report mode keeps for each input report it reads.

use crate::report::{Held, Mouse};

/// An input report in report mode: its value handle, its service's layout
/// and its Report ID, whether it holds keys or a pointer, and what it
/// last said was held.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ReportIn {
    pub value: u16,
    pub layout: usize,
    pub id: u8,
    pub keys: bool,
    pub points: bool,
    pub held: Held,
    pub mouse: Mouse,
}
