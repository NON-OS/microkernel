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

//! Who this driver serves (C6). The card holds whatever the person keeps
//! on it, and maybe a store this machine trusts, so only the kernel's block
//! layer (pid 0, which the kernel never hands a process) and a sender the
//! kernel says holds StoreWrite (the installer) reach the medium, as the
//! SATA, NVMe and virtio-blk drivers rule. A health check stays open. The
//! kernel is asked on every request: a verdict kept would outlive its
//! holder and pass to the next process given the pid.

use nonos_libc::mk_cap_check;

use crate::protocol::OP_HEALTHCHECK;

/// StoreWrite, as abi/caps.toml numbers it.
const CAP_STORE_WRITE: u64 = 1 << 26;

/// Pure, so the host proofs walk every case.
pub fn allows(op: u16, sender: u32, holds_store_write: bool) -> bool {
    op == OP_HEALTHCHECK || sender == 0 || holds_store_write
}

pub fn permits(op: u16, sender: u32) -> bool {
    allows(op, sender, false) || allows(op, sender, mk_cap_check(sender, CAP_STORE_WRITE))
}
