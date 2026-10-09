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

//! The DEC control-sequence state machine, after Paul Williams' description
//! of the VT500 parser, with UTF-8 in the ground state.

mod advance;
mod csi_states;
mod dcs_states;
mod escape;
mod ground;
mod handler;
mod machine;
mod seq;
mod strings;

pub use handler::Handler;
pub use machine::Parser;
pub use seq::Seq;
