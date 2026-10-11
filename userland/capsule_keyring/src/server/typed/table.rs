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

//! Everything the keyring will sign as typed data. A message not described
//! here cannot be signed through OP_SIGN_TYPED_DATA, whatever its caller
//! says it is: no Permit, no Permit2, no order book, no approval.

use super::bond::{PUBLISHER_BINDING, RELEASE_ATTEST};
use super::entry::Entry;
use super::voucher::VOUCHER;

pub static ENTRIES: [&Entry; 3] = [&VOUCHER, &PUBLISHER_BINDING, &RELEASE_ATTEST];

pub fn entry(id: u16) -> Option<&'static Entry> {
    ENTRIES.iter().copied().find(|e| e.id == id)
}
