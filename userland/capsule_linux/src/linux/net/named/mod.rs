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

//! Unix sockets with names: the sockaddr_un a guest gives, the names a
//! family socket is bound to, and bind and connect on them.

mod addr;
mod auto;
mod bind;
mod connect;
mod display;
mod name;

pub use addr::{read as read_uaddr, UAddr};
pub use bind::bind;
pub use connect::connect;
pub use name::{find, resolve};
