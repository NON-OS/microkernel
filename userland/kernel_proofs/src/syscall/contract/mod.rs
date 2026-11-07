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

// The real syscall capability table. `is_allowed` is `pub(super)`, visible to
// this parent module; expose it for the proofs.
#[path = "../../../../../src/syscall/contract/cap_table/mod.rs"]
mod cap_table;

#[cfg(any(test, feature = "fuzz"))]
#[path = "../../../../../src/syscall/contract/allowlist/mod.rs"]
pub mod allowlist;

#[cfg(test)]
mod allowlist_tests;

// The resolver the syscall gate runs, and its argument type.
#[cfg(test)]
#[path = "../../../../../src/syscall/contract/args.rs"]
pub mod args;
#[cfg(test)]
#[path = "../../../../../src/syscall/contract/resolver/mod.rs"]
mod resolver;

#[cfg(test)]
mod gate_race_tests;

// What a refusal costs: EPERM, or the end of a native capsule.
#[cfg(test)]
#[path = "../../../../../src/syscall/contract/deny/mod.rs"]
mod deny;
#[cfg(test)]
mod deny_tests;

#[cfg(test)]
mod admin_split_tests;

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

/// The table as it answers any caller but the desktop shell.
pub fn is_allowed(caps: &CapabilityToken, number: SyscallNumber) -> bool {
    cap_table::is_allowed(caps, number, false)
}

/// The table as it answers the desktop shell the kernel spawned.
pub fn is_allowed_as_shell(caps: &CapabilityToken, number: SyscallNumber) -> bool {
    cap_table::is_allowed(caps, number, true)
}

/// The allowlist a process holds for `caps`, as install derives it: the token
/// signed with this crate's key, its MAC checked, then the tables asked
/// (`allowlist::for_token`, crate-private there). For the fuzz target.
#[cfg(any(test, feature = "fuzz"))]
pub fn allowlist_for(caps: &CapabilityToken) -> allowlist::Allowlist {
    crate::capabilities::token::test_key();
    let mut signed = caps.clone();
    if crate::capabilities::token::sign_token(&mut signed).is_err() {
        return allowlist::Allowlist::EMPTY;
    }
    allowlist::for_token(&signed)
}

/// The list install derives for a token whose MAC does not check.
#[cfg(any(test, feature = "fuzz"))]
pub fn allowlist_for_unsigned(caps: &CapabilityToken) -> allowlist::Allowlist {
    crate::capabilities::token::test_key();
    allowlist::for_token(caps)
}
