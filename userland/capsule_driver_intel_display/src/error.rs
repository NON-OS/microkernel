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

//! Why one bring-up attempt failed, in the words the give-up line carries.

use nonos_intel_display::engine::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntelError {
    /// The broker refused a claim, a mapping, a grant or a config access.
    Broker(i64),
    /// GGC names no GGTT: firmware gave the engine no table to scan through.
    NoGgtt,
    /// The engine refused (nothing lit, a layout it does not flip, a
    /// latch that did not come); the display is as firmware left it.
    Engine(Error),
}

pub type IntelResult<T> = Result<T, IntelError>;

pub const fn reason(e: IntelError) -> &'static str {
    match e {
        IntelError::Broker(_) => "intel display: broker refused a claim, grant or map",
        IntelError::NoGgtt => "intel display: GGC names no GGTT",
        IntelError::Engine(Error::NothingLit) => "intel display: firmware lit no pipe",
        IntelError::Engine(Error::Layout) => "intel display: firmware's plane is not flippable",
        IntelError::Engine(Error::GgttFull) => "intel display: no room in the GGTT",
        IntelError::Engine(Error::Timeout) => "intel display: the plane did not latch",
        IntelError::Engine(_) => "intel display: the engine refused the takeover",
    }
}
