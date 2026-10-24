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

//! Display timings computed one way for every driver. A mode EDID lists
//! in full is taken as it is; one it names by size and rate alone is
//! looked up in the DMT, else computed by CVT 1.2.

#![no_std]

pub mod check;
pub mod cvt;
pub mod dmt;
pub mod dmt_parse;
pub mod fit;
pub mod lex;
pub mod link;
pub mod make;
pub mod resolve;

pub use check::{clock_ok, in_range};
pub use cvt::{cvt, Blanking};
pub use dmt::{dmt, Dmt, Kind};
pub use fit::{fit, Range};
pub use link::{carries, Link};
pub use nonos_edid::Detailed;
pub use resolve::{compute, resolve};
