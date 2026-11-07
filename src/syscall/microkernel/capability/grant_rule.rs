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

//! The rules of `sys_cap_grant` and `sys_cap_revoke`: the caller's word on
//! one side, the target's envelope on the other. Kept pure so
//! `userland/kernel_proofs` checks the same functions the handlers run,
//! against the preconditions `Effect.capGrant` and `Effect.capRevoke` state
//! in `formal/Formal/Step.lean`.

/// Grant: the caller holds `admin` and every bit of `mask`.
#[inline]
pub(crate) const fn may_grant(held: u64, admin: u64, mask: u64) -> bool {
    held & admin != 0 && held & mask == mask
}

/// The target side of a grant: every bit of `mask` lies within the target's
/// envelope (`process::caps_envelope`), so no grant lifts a process above
/// what its manifest and the image ceiling admitted it to.
#[inline]
pub(crate) const fn within_envelope(mask: u64, envelope: u64) -> bool {
    mask & !envelope == 0
}

/// `sys_cap_grant`'s whole decision once the caller's word and the target's
/// envelope are looked up: None for either is a missing process, refused.
#[inline]
pub(crate) const fn grant_decision(
    caller_held: Option<u64>,
    target_envelope: Option<u64>,
    admin: u64,
    mask: u64,
) -> bool {
    match (caller_held, target_envelope) {
        (Some(held), Some(envelope)) => {
            may_grant(held, admin, mask) && within_envelope(mask, envelope)
        }
        _ => false,
    }
}

/// Whether the caller may revoke from the target, once its Admin standing is
/// known (`may_revoke`). Revoking from one's own process needs nothing: it
/// only narrows. Otherwise the caller needs Admin and must be init or the
/// target's parent; a protected service (compositor, input router, the net
/// stack, policy) may be revoked from by init alone.
#[inline]
pub(crate) const fn revoke_decision(
    same_process: bool,
    caller_admin: bool,
    caller_is_init: bool,
    caller_is_parent: bool,
    target_protected: bool,
) -> bool {
    if same_process {
        return true;
    }
    if !caller_admin {
        return false;
    }
    if target_protected {
        return caller_is_init;
    }
    caller_is_init || caller_is_parent
}

/// Revoke: the caller holds `admin`.
#[inline]
pub(crate) const fn may_revoke(held: u64, admin: u64) -> bool {
    held & admin == admin
}
