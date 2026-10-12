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
//! The USB Video Class (UVC 1.1 and 1.5, every layout as recalled,
//! unverified against the specification): what a camera's configuration
//! says it can stream, the probe and commit controls that agree a stream,
//! the alternate setting with room for it, and frames put back together
//! from the payloads its endpoint carries. Pure, so the proofs hold it.

#![no_std]

extern crate alloc;

mod alt;
mod desc;
mod frame;
mod frames;
mod header;
mod probe;
mod push;
mod vs;
mod walk;

pub use alt::{pick_alt, Alt};
pub use desc::{Camera, Format, FormatKind, FrameSize};
pub use frame::{Assembler, Payload};
pub use probe::{Probe, CONTROL_LEN_11, GET_CUR, SET_CUR, VS_COMMIT, VS_PROBE};
pub use walk::camera;
