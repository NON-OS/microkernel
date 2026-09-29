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

//! The program a machine runs when its store names none.

use alloc::vec::Vec;

use super::launch::Launch;
use super::origin::Origin;

/// The built-in program: Alpine's static busybox, embedded so a machine with
/// nothing in the store still runs a real Linux binary.
static BUILT_IN: &[u8] = include_bytes!("../../guests/busybox.elf");

pub(super) fn built_in() -> Launch {
    Launch {
        path: b"/bin/busybox".to_vec(),
        bytes: BUILT_IN.to_vec(),
        origin: Origin::BuiltIn,
        args: Vec::new(),
    }
}
