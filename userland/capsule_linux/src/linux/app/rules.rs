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

//! What a box allows, decided from the policy alone, so each rule is held
//! by the host proofs as it ships. gate.rs and calls.rs ask these.

use nonos_linux_policy::market::ALWAYS_SERVED;
use nonos_linux_policy::mask::{grants, NET_ANON, NET_LISTEN_LO};
use nonos_linux_policy::SyscallSet;

/// Whether call `number` is refused before it is served. exit and
/// exit_group are served whatever the set: refused, the thread would run on
/// past its own end, and nothing it could do there is safer. A market policy
/// must name both (nonos_linux_policy::market_ready).
pub fn call_refused(calls: &SyscallSet, number: u64) -> bool {
    !ALWAYS_SERVED.contains(&number) && !calls.allows(number)
}

/// An internet socket at all: for reaching out, or for listening at home.
pub fn inet_allowed(mask: u64) -> bool {
    grants(mask, NET_ANON) || grants(mask, NET_LISTEN_LO)
}

/// What a run does once its program's policy is bound, by whether the box
/// has NET_ANON and whether this role holds Network (plan/32).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NetStep {
    /// Run here: the role holds Network exactly when the box asks for it.
    Run,
    /// The run role holds no Network: ask to be started again with it.
    AskAgain,
    /// The network role, for a box without NET_ANON: run nothing.
    Refuse,
}

pub fn net_step(mask: u64, holds_network: bool) -> NetStep {
    match (grants(mask, NET_ANON), holds_network) {
        (true, false) => NetStep::AskAgain,
        (false, true) => NetStep::Refuse,
        _ => NetStep::Run,
    }
}
