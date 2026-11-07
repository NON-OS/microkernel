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

use alloc::string::String;
use core::fmt::Write;

use crate::syscall::numbers::SyscallNumber;

/// The serial line a kill writes: which capsule, which call, and the
/// declared bits its allowlist is derived from.
pub(in crate::syscall::contract) fn kill_line(
    pid: u32,
    capsule: &str,
    number: SyscallNumber,
    envelope: u64,
) -> String {
    let mut out = String::new();
    let _ = write!(
        out,
        "[CAP-KILL] pid={} capsule={} syscall={:?}({:#x}) allowlist=manifest:{:#x}",
        pid, capsule, number, number as u64, envelope
    );
    out
}
