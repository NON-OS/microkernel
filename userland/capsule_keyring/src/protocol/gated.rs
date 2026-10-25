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

//! The ops the wallet alone may ask, whatever key they name: sealing and
//! opening its vault record and its shield seeds, and reading the recovery
//! words. Each also checks that the caller owns the key; this asks who the
//! caller is as well, so a capsule that came to own a words entry still
//! cannot read the words out.

use super::types::{OP_SHIELD_MATERIAL, OP_SHIELD_OPEN, OP_SHIELD_SEAL};
use super::types::{OP_VAULT_OPEN, OP_VAULT_SEAL};

pub const WALLET_ONLY: [u16; 5] =
    [OP_VAULT_SEAL, OP_VAULT_OPEN, OP_SHIELD_MATERIAL, OP_SHIELD_SEAL, OP_SHIELD_OPEN];

/// Whether `op` is asked of the vault gate before it is served.
pub fn wallet_only(op: u16) -> bool {
    WALLET_ONLY.contains(&op)
}
