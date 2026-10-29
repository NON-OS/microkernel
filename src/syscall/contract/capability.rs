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

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

use super::args::SyscallArgs;
use super::resolver::{resolve as resolver_resolve, ResolveContext};

/// Witness that a syscall has passed the capability check for a specific
/// `SyscallNumber` plus argument set.
///
/// The only constructor is `Capability::resolve`. User-space code cannot
/// construct one; in-kernel code outside the contract module also cannot,
/// because the wrapped token field is private. A handler that takes
/// `Capability` therefore has executable proof that the check ran.
pub struct Capability {
    token: CapabilityToken,
}

impl Capability {
    pub fn resolve(number: SyscallNumber, args: &SyscallArgs) -> Option<Self> {
        let proc = crate::process::current_process()?;
        let token_arc = proc.capability_token_arc();
        let ctx = ResolveContext {
            current_asid: crate::memory::paging::manager::lookup_asid_for_process(proc.pid)
                .unwrap_or(0),
            boot_session_nonce: crate::security::boot_session::nonce(),
            capsule_revocation_epoch: proc.revocation_epoch(),
            caller_launches,
        };
        resolver_resolve(&token_arc, number, args, &ctx).ok()?;
        Some(Self { token: (*token_arc).clone() })
    }

    /// Borrow the underlying capability token. Handlers that need to
    /// inspect specific bits reach the token through here.
    #[inline]
    pub fn token(&self) -> &CapabilityToken {
        &self.token
    }
}

/// Whether the caller owns an endpoint the kernel registered at spawn, which
/// no capsule may claim for itself (register_allowed.rs), and not a process
/// name: the desktop shell's, or a capsule's SERVES pairs with a listing.
fn caller_launches() -> bool {
    use crate::services::registry::{caller_owns, serves::LISTINGS};
    caller_owns("desktop_shell") || LISTINGS.iter().any(|&(capsule, _)| caller_owns(capsule))
}
