// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Who the block clients serve, pure: a driver, the holder of the store
//! (vfs persisting packages), or the data volume's own I/O, which the kernel
//! does for a data syscall that has already checked its caller (FileSystem,
//! StreamImport) and which stays in the volume's window and plan. A capsule
//! reaching a disk any other way is refused, as before.
//!
//! No crate paths here, so the host proofs mount this file as it is.

pub(crate) const fn admits(volume_io: bool, driver: bool, storage: bool) -> bool {
    driver || storage || volume_io
}
