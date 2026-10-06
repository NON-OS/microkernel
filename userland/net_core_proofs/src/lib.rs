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

//! Host proofs for net.core: the request decode any client can reach, DNS
//! lookups, autojoin, the link and lease watches, and TCP liveness. The
//! `#[path]` includes pull in the real source by the `crate::` paths it uses.

// The wire's errnos, magics and ops, which the decode names.
#[path = "../../capsule_net_core/src/protocol/mod.rs"]
pub mod protocol;

// The header decode, the refusal and the reply; not the handlers or loop.
pub mod server;

// One DNS lookup whose caller waits, which the table below holds.
#[path = "../../capsule_net_core/src/server/handlers/dns/waiting.rs"]
pub mod waiting;

// The DNS lookups whose callers wait for an answer given after a poll.
#[path = "../../capsule_net_core/src/server/handlers/dns/pending.rs"]
pub mod dns_pending;

// What a lookup asked of every DNS server comes to.
#[path = "../../capsule_net_core/src/server/handlers/dns/verdict.rs"]
pub mod dns_verdict;

// When autojoin scans, hands a join to the driver, or watches one run.
#[path = "../../capsule_net_core/src/autojoin/machine.rs"]
pub mod autojoin_machine;

// When to say DHCP has not given the bound interface a lease.
#[path = "../../capsule_net_core/src/iface/lease_wait.rs"]
pub mod lease_wait;

// What the bound link did since it was last asked.
#[path = "../../capsule_net_core/src/iface/link_watch.rs"]
pub mod link_watch;

// How long a TCP connection may go unanswered before it is given up.
#[path = "../../capsule_net_core/src/server/handlers/tcp/connect/liveness.rs"]
pub mod tcp_liveness;

#[cfg(test)]
mod autojoin_tests;
#[cfg(test)]
mod dns_errno_tests;
#[cfg(test)]
mod dns_pending_tests;
#[cfg(test)]
mod dns_verdict_tests;
#[cfg(test)]
mod lease_wait_tests;
#[cfg(test)]
mod link_watch_tests;
#[cfg(test)]
mod refusal_tests;
#[cfg(test)]
mod tcp_link;
#[cfg(test)]
mod tcp_liveness_tests;
#[cfg(test)]
mod tcp_pair;
