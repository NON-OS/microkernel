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
 * The pins, compiled in from the Linux personality's own tables, the ones
 * its measurement covers: a catalogue entry that is not one of them, byte
 * for byte, is refused before anything is fetched.
 */

#[path = "../../../capsule_linux/src/linux/file/models/hex.rs"]
mod hex;
#[path = "../../../capsule_linux/src/linux/file/models/pinned.rs"]
mod pinned;
#[path = "../../../capsule_linux/src/linux/file/models/pinned_coder.rs"]
mod pinned_coder;
#[path = "../../../capsule_linux/src/linux/file/models/pinned_qwen25.rs"]
mod pinned_qwen25;
#[path = "../../../capsule_linux/src/linux/file/models/pinned_qwen25_big.rs"]
mod pinned_qwen25_big;
#[path = "../../../capsule_linux/src/linux/file/models/pinned_qwen3.rs"]
mod pinned_qwen3;

pub use pinned::{all, keepable, pin_of};
