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

//! Which ioctl requests a descriptor answers. The file requests are any
//! descriptor's; the terminal's are a console's, and only while the family
//! is on a terminal; everything else is ENOTTY, which is how a program
//! learns a descriptor is not a terminal. Pure, so the host proofs hold the
//! table for every kind of descriptor.

mod args;
mod codes;
mod decode;
mod req;

pub use args::{flow_ok, flush_input, winsize};
pub use decode::request;
pub use req::Req;
