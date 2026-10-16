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

//! The driver's lines on the kernel console, which `log rtsx` shows on a
//! machine with no serial port. Printed only when the build grants the
//! Debug capability; otherwise the call is refused and the driver goes on.
//! No line names the card beyond its size: its CID stays out (C5).

mod emit;
pub mod line;

pub use emit::{emit, failed, step};
pub use line::Line;
