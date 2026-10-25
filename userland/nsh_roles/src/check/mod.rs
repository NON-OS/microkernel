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

//! The checks a login and an escalation share: the issuer, the grant's
//! time, the revocation list, the role and the capabilities. `Host` is what
//! the server knows when it checks: its trusted table, its trusted list and
//! the time now, read from the host clock by the caller.

pub mod approvals;
pub mod escalate;
mod host;
pub mod login;

pub use host::Host;
pub(crate) use host::{revocation, role};

use crate::hybrid::verify;
use crate::{CapSet, Refusal};
use nsh_crypto::{labelled, Label};
use nsh_wire::Grant;

/// The issuer is a role authority and both its signatures verify over
/// `H("nsh/1 grant\0" || signed bytes)`. `unknown` and `bad` say how each
/// failure is told.
pub(crate) fn issuer(
    host: &Host<'_>,
    g: &Grant<'_>,
    unknown: Refusal,
    bad: Refusal,
) -> Result<(), Refusal> {
    let key = host.policy.authority(g.issuer).ok_or(unknown)?;
    verify(&key.public, &labelled(Label::Grant, g.signed), &g.sig).then_some(()).ok_or(bad)
}

/// `not_before <= now < not_after`.
pub(crate) fn time(host: &Host<'_>, g: &Grant<'_>) -> Result<(), Refusal> {
    if host.now_ms < g.not_before_ms {
        return Err(Refusal::NotYet);
    }
    match host.now_ms < g.not_after_ms {
        true => Ok(()),
        false => Err(Refusal::Expired),
    }
}

/// Every name is known, and the set is within `ceiling`.
pub(crate) fn caps(g: &Grant<'_>, ceiling: CapSet) -> Result<CapSet, Refusal> {
    let set = CapSet::named(&g.caps).map_err(|_| Refusal::CapUnknown)?;
    set.within(ceiling).then_some(set).ok_or(Refusal::CapsCeiling)
}
