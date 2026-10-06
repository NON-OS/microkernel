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

//! nonos_route_link's pure parts, compiled from the files that ship. The
//! modules sit at the crate root under their own names, as they do in the
//! crate, so their `crate::` paths resolve to the same files here.

extern crate alloc;

#[path = "../../nonos_route_link/src/answer.rs"]
pub mod answer;
#[path = "../../nonos_route_link/src/bounds.rs"]
pub mod bounds;
#[path = "../../nonos_route_link/src/carrier.rs"]
pub mod carrier;
#[path = "../../nonos_route_link/src/describe.rs"]
pub mod describe;
#[path = "../../nonos_route_link/src/direct_only.rs"]
pub mod direct_only;
#[path = "../../nonos_route_link/src/frame.rs"]
pub mod frame;
#[path = "../../nonos_route_link/src/opening.rs"]
pub mod opening;
#[path = "../../nonos_route_link/src/pick.rs"]
pub mod pick;
#[path = "../../nonos_route_link/src/refusal.rs"]
pub mod refusal;
#[path = "../../nonos_route_link/src/slice.rs"]
pub mod slice;
#[path = "../../nonos_route_link/src/socks.rs"]
pub mod socks;
#[path = "../../nonos_route_link/src/tunnel.rs"]
pub mod tunnel;
#[path = "../../nonos_route_link/src/tunnel_io.rs"]
pub mod tunnel_io;

/* net.ntp's decision, made from the rule above before every exchange. */
#[path = "../../capsule_net_ntp/src/decide.rs"]
pub mod ntp_decide;

/// The app SDK's rule for which network a connection leaves through, over
/// this crate's own `pick`.
pub mod sdk_net;

#[cfg(test)]
mod fake;

#[cfg(test)]
mod answer_tests;
#[cfg(test)]
mod direct_tests;
#[cfg(test)]
mod frame_tests;
#[cfg(test)]
mod ntp_tests;
#[cfg(test)]
mod pick_tests;
#[cfg(test)]
mod sdk_way_tests;
#[cfg(test)]
mod slice_tests;
#[cfg(test)]
mod socks_tests;
#[cfg(test)]
mod tunnel_fault_tests;
#[cfg(test)]
mod tunnel_open_tests;
#[cfg(test)]
mod tunnel_read_tests;
