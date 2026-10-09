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

/*
 * /models: model files on the machine's data volume, read-only.
 *
 * Each name here is a file on the sealed volume, reached through the
 * kernel by name. A model the personality pins is imported the first time
 * it is opened, and kept only if its SHA-256 is the pinned one.
 */

mod catalog;
mod direct;
mod first_use;
mod held;
mod hex;
mod name;
mod open;
mod pinned;
mod pinned_coder;
mod pinned_qwen25;
mod pinned_qwen25_big;
mod pinned_qwen3;
mod read;
mod size;
mod stat;
mod stat_size;
mod verified;

pub use direct::read_into;
pub use held::held;
pub use open::open;
pub use read::read;
pub use size::size_of as ensure;
pub use stat::stat;
