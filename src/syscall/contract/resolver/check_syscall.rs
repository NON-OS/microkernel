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

use crate::syscall::contract::allowlist::{decide, Allowlist};
use crate::syscall::numbers::SyscallNumber;

use super::error::ResolverError;

/*
 * A bit test against the list derived from the token at install, which
 * gives the answer the capability tables give (proved for every syscall in
 * kernel_proofs). The launcher question is asked only for the syscalls
 * whose answer turns on it.
 */
pub(super) fn check_syscall_allowed(
    list: &Allowlist,
    number: SyscallNumber,
    launcher: fn() -> bool,
) -> Result<(), ResolverError> {
    if !decide(list, number as u64, launcher) {
        return Err(ResolverError::SyscallNotPermitted);
    }
    Ok(())
}
