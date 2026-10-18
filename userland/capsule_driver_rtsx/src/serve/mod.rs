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

//! The card served as a block device on driver.rtsx0, to the kernel's
//! block layer and the installer alone, with the slot watched between and
//! before requests so a card swapped in is never served as the one before.

pub mod admits;
mod answer;
mod blocks;
mod change;
pub mod fault;
mod identity;
pub mod medium;
mod reply;
pub mod request;
pub mod run;
pub mod slot;

pub use run::run;
