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

//! The gate a send passes: the list an endpoint is held to (held.rs says
//! why), or the capsule the kernel held it to at run time.
//!
//! A Linux run's served endpoint carries the listener of a guest that serves
//! one capsule (SERVE_ONE, serve_rule.rs), so the kernel holds it to that
//! capsule: only the process owning the capsule's endpoint reaches it, by
//! name, port or pid, and no other capsule, run or peer does. A list comes
//! first, so a hold never opens a held driver; an endpoint neither listed nor
//! held is reached as it always was.

use super::callers_of;

/// Whether a caller may send to `endpoint`, held at run time to `held_to`,
/// given `owns`, which says whether the caller owns an endpoint of that name.
pub(crate) fn endpoint_admits(
    endpoint: &str,
    held_to: Option<&str>,
    owns: impl Fn(&str) -> bool,
) -> bool {
    match callers_of(endpoint) {
        Some(callers) => callers.iter().any(|c| owns(c)),
        None => held_to.is_none_or(owns),
    }
}

/// Whether a caller holding `held` may write straight into the inbox of a
/// process serving the endpoints `served`, each with its requirement and its
/// run-time hold.
///
/// Every endpoint a process serves is read from that one inbox, so a send by
/// pid is a send to each of them and must pass each one's gate: its
/// capabilities, its list and its hold. Before this, MkIpcSendToPid checked
/// none, and a capsule without Network reached net.sockets, or a card's
/// driver, by looking up its pid.
pub(crate) fn inbox_admits<'a>(
    served: impl IntoIterator<Item = (&'a str, u64, Option<&'a str>)>,
    held: u64,
    owns: impl Fn(&str) -> bool,
) -> bool {
    served.into_iter().all(|(name, required, held_to)| {
        held & required == required && endpoint_admits(name, held_to, &owns)
    })
}
