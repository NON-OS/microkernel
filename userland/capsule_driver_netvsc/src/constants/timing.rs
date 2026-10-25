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

//! How long the driver waits on the host.

/// Each step of the start: a protocol message's completion, an RNDIS reply.
pub const INIT_MS: u64 = 10_000;
/// How long the driver waits for the VMBus core to give it its channel.
pub const CHANNEL_MS: u64 = 15_000;
/// A send waits this long for a free send buffer section.
pub const SEND_MS: u64 = 1_000;
/// The station-address probe: DHCP DISCOVERs this far apart, this many,
/// then a last wait as long, before the host-assigned address is taken.
pub const PROBE_EVERY_MS: u64 = 2_000;
pub const PROBE_TRIES: u32 = 3;
