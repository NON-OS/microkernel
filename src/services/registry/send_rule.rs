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

//! The rule of each gate an addressed send passes, in the order
//! send_address.rs as_addressed takes them. Each kernel gate gathers its
//! facts and decides by its rule here:
//!
//! 1. `endpoint_caps`: send_caps.rs caller_satisfies_endpoint;
//! 2. `reaches`: peers_check.rs caller_may_reach;
//! 3. `writes_inbox`, under addressed.rs addressed_admits: peers_inbox.rs
//!    caller_may_write_inbox, which send_address.rs caller_may_address and
//!    MkIpcSendToPid both call;
//! 4. `routes`: ipc/kernel_ipc.rs, the last check before the message is
//!    queued.
//!
//! Every capability test among them is policy.rs caps_admit, so no gate
//! refuses a caller another admits by a different rule.
//!
//! Pure, and walked whole in kernel_proofs (ipc_held_tests::send_walk).

use super::held::{endpoint_admits, inbox_admits};
use super::peers::may_reach;
use super::caps_admit;

/// The capability gate of the endpoint addressed, which asks `required` of
/// a caller holding `held`. A requirement of zero is an endpoint never
/// finished being set up, and is refused.
pub(crate) fn endpoint_caps(target: &str, required: u64, held: u64) -> bool {
    required != 0 && caps_admit(target, required, held)
}

/// The lists capabilities cannot express: the callers `target` is held to,
/// at build time or at run time (`held_to`), and the peer list of the
/// calling process, named `caller`, or None when it has no name.
pub(crate) fn reaches(
    target: &str,
    held_to: Option<&str>,
    caller: Option<&str>,
    owns: impl Fn(&str) -> bool,
) -> bool {
    endpoint_admits(target, held_to, owns) && caller.is_none_or(|name| may_reach(name, target))
}

/// The gate of every endpoint the receiving process serves, since all of
/// them are read from the one inbox the send lands in, for a caller holding
/// `held`.
pub(crate) fn writes_inbox<'a>(
    served: impl IntoIterator<Item = (&'a str, u64, Option<&'a str>)>,
    held: u64,
    owns: impl Fn(&str) -> bool,
) -> bool {
    inbox_admits(served, |name, required| caps_admit(name, required, held), owns)
}

/// The route's own check: the capabilities the kernel keeps for the sending
/// process, `held`, or None when it has ended, against the endpoint's
/// requirement.
pub(crate) fn routes(target: &str, required: u64, held: Option<u64>) -> bool {
    held.is_some_and(|held| caps_admit(target, required, held))
}
