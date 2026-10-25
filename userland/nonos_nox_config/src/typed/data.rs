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

use super::{PUBLISHER_BINDING, RELEASE_ATTEST, VOUCHER};
use crate::domain::Domain;

/// One EIP-712 struct the OS signs: its type string, the typehash pinned from
/// it, the domain it is signed under and how many 32-byte words it encodes to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TypedData {
    pub type_string: &'static str,
    pub typehash: [u8; 32],
    pub domain: &'static Domain,
    pub fields: usize,
}

/// Every struct the OS signs, in the order of the keyring's table (plan/31).
pub const TYPED: [&TypedData; 3] = [&VOUCHER, &PUBLISHER_BINDING, &RELEASE_ATTEST];
