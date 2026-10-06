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

//! CDC-NCM (USB NCM 1.0): finding the function, binding it, the sizes its
//! NTB parameters give, and its transfer blocks out and in.

pub mod align;
pub mod bind;
pub mod function;
mod init;
pub mod limits;
pub mod link;
pub mod ntb_in;
pub mod ntb_out;
pub mod pad;
pub mod params;
pub mod quirk;
mod requests;
mod setup;
pub mod shape;
mod start;
mod transfer;
pub mod verify;

pub use bind::bind;
