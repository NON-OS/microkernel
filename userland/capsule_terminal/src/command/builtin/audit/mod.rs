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

//! `audit`: who on this machine can touch the data, the network or the
//! hardware, right now. A join of the kernel's process table and its
//! attestation registry, read the way `receipt` reads it; nothing here is
//! a capsule's account of itself.

mod agent;
mod format;
mod join;
pub(crate) mod names;
mod reach;
mod read;
mod run;
pub(crate) mod table;
pub mod withheld;

pub use run::run;
