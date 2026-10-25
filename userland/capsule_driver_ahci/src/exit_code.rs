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

//! The status the driver leaves with when its bring-up runs out (main.rs):
//! a controller that came up with no ATA disk behind it on every attempt is
//! EXIT_EMPTY, a clean "no disk"; any other give-up stays EXIT_GAVE_UP. Pure,
//! so the host proofs hold it.

use nonos_libc::bringup::{EXIT_EMPTY, EXIT_GAVE_UP};

use crate::error::AhciError;

/// The code to leave with for start's `code`, the last attempt having
/// failed with `last`.
pub fn exit_code(code: i32, last: Option<AhciError>) -> i32 {
    match (code, last) {
        (EXIT_GAVE_UP, Some(AhciError::NoDisk)) => EXIT_EMPTY,
        _ => code,
    }
}
