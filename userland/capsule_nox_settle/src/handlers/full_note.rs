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
//! One line to the owner the first time a chain's cache goes full: from
//! then each miss reads that chain's whole history again (v1.4.63 ruling).
//! It names the chain and the cache, never an order, listing or key.

use alloc::format;

pub(super) fn said_full(cache: &str, chain_id: u64) {
    let line = format!(
        "[nox_settle] chain {chain_id}: the {cache} cache is full; each miss now reads          the whole history again\n"
    );
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
