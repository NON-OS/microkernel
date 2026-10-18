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

//! LE privacy (Vol 6, Part B, 1.3.2.2 and 1.3.2.3): a resolvable private
//! address is made from an identity resolving key and resolved with it, so
//! only a bonded peer can tell two of this host's addresses apart.

mod ah;
mod rpa;

pub use ah::ah;
pub use rpa::{prand, resolves, rpa};
