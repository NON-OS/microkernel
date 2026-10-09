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

//! RNDIS: finding the function, the control channel, binding, and the
//! packet framing on the bulk pipes.

pub mod batch;
pub mod bind;
mod configure;
pub mod control;
mod data_iface;
pub mod function;
mod halt;
pub mod init;
pub mod link;
pub mod message;
pub mod packet;
pub mod query;
pub mod reply;
pub mod set;
mod transfer;

pub use bind::bind;
