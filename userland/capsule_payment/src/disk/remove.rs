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

//! Letting a file go: an offer an order took, or an order's escrow files
//! once the order leaves. A file already gone is no error.

use super::ops;

pub fn remove(path: &[u8]) -> Result<(), i32> {
    match ops::unlink(path) {
        Ok(()) | Err(Some(super::vfs::ENOENT)) => Ok(()),
        Err(e) => Err(e.unwrap_or(nonos_payment_proto::EAGAIN)),
    }
}
