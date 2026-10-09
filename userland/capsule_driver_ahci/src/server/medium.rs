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

//! The medium rule (`medium_rule`), with the kernel asked on every request:
//! a cached verdict would outlive the holder's exit and follow its pid to
//! whatever process is handed that pid next.

use nonos_libc::mk_cap_check;

use super::medium_rule::allows;

/// StoreWrite, as abi/caps.toml numbers it.
const CAP_STORE_WRITE: u64 = 1 << 26;

pub fn permits(op: u16, sender_pid: u32) -> bool {
    allows(op, sender_pid, false) || allows(op, sender_pid, mk_cap_check(sender_pid, CAP_STORE_WRITE))
}
