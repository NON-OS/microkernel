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

//! A grant acted on while its record stands (L77). Unbind and both drains
//! take the record out under the records' lock before their teardown masks
//! the line, so a mask after a removal always lands after what `f` wrote.

use super::super::grant::IrqGrant;
use super::store::RECORDS;

pub(in super::super) fn with_grant<R>(grant_id: u64, f: impl FnOnce(&IrqGrant) -> R) -> Option<R> {
    RECORDS.lock().iter().find(|g| g.grant_id == grant_id).map(f)
}
