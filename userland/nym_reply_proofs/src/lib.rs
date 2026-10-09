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

//! Host proofs for the reply path of net.nym. Includes the real capsule
//! source via `#[path]` and drives it with hand-built fragments, the way a
//! network requester's answer arrives after the mixnet has reordered,
//! duplicated or lost pieces of it. The session table those replies land in
//! is held here too: one client's share of it, and the closing of sessions
//! whose client ended.

extern crate alloc;

pub mod crypto;
pub mod message;
pub mod packet;
pub mod protocol;
pub mod reply;
pub mod server;
pub mod sphinx;
pub mod state;
pub mod surb;
pub mod topology;
pub mod ws_frame;

/// The record of fragments sent and not yet acknowledged, and the mix delay
/// they are sent with: both pure, compiled in from net.nym as they ship.
#[path = "../../capsule_net_nym/src/ack/ledger.rs"]
pub mod ack_ledger;
#[path = "../../capsule_net_nym/src/mixnet/exp_delay.rs"]
pub mod exp_delay;

/// The reader on the other side of a batched read: net.socks5 putting the
/// records back into messages.
#[path = "../../capsule_socks5/src/nym/batch.rs"]
pub mod socks5_batch;

#[cfg(test)]
mod blocks_tests;
#[cfg(test)]
mod delay_tests;
#[cfg(test)]
mod delivery_tests;
#[cfg(test)]
mod frame_tests;
#[cfg(test)]
mod ledger_tests;
#[cfg(test)]
mod reassembly_tests;
#[cfg(test)]
mod refusal_tests;
#[cfg(test)]
mod session_share_tests;
#[cfg(test)]
mod surb_tests;
