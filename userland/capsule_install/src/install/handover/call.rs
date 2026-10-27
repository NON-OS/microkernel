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

//! The kernel call that hands the data volume over for an update. It is
//! `nonos_libc::mk_data_handover` (`userland/libc/src/data.rs`), which needs
//! StoreWrite and FileSystem, both of which this capsule holds. Until that
//! call is in the libc this branch builds against, this stand-in answers
//! ENOSYS, and the screen says the system cannot prepare yet. When it is,
//! the function below becomes `pub use nonos_libc::mk_data_handover;`.

/// 0 when the volume is handed over; a negative errno otherwise.
pub fn mk_data_handover() -> i64 {
    -38
}
