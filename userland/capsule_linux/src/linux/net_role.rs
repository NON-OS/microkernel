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

//! Whether this personality was started in a role that holds Network: the
//! terminal's network role (app.linux.term.net) or the run's (run.net).

use nonos_libc::{mk_cap_check, mk_getpid};

/// Capability::Network, bit 2 of the kernel's capability mask.
const NETWORK: u64 = 1 << 2;

pub fn held() -> bool {
    mk_cap_check(mk_getpid(), NETWORK)
}
