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

//! What the compositor keeps of a driver it drives on the wire: whose
//! primary it shows, which driver process served it, and what CAPS said.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireLink {
    /// The service the driver announced, from the kernel's display list.
    pub name: &'static str,
    /// The serial of the primary attached now (pid << 16 | counter).
    pub serial: u32,
    /// The driver's pid when it was looked up; a new one is a restart.
    pub pid: u32,
    /// CAPS' physical size, w | h << 16 mm, 0 unknown.
    pub physical_mm: u32,
    /// CAPS' output count; a poll that reads another is a hot-plug.
    pub outputs: u16,
    /// A FLUSH answered ESTALE: the primary must be asked for again
    /// before anything else is shown.
    pub stale: bool,
    /// Where the primary's cursor slot starts, when CAPS has HW_CURSOR and
    /// the pointer goes on the device's plane; 0 composites it.
    pub cursor_offset: u32,
    /// The driver said its device left (OUTPUT_NOTICE): drive another.
    pub leaving: bool,
    /// The driver asked for a poll now (OUTPUT_NOTICE).
    pub nudged: bool,
    /// CAPS has no MODE_SET: an output it says is LOST stays lost.
    pub fixed: bool,
}
