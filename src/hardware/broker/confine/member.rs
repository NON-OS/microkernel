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

//! A device in a claim's isolation group (K11b): what join.rs holds to one
//! capsule, and names when it refuses.

/// Its device_id, its bus, device and function, and those of the hop it
/// reaches the claimed device past: a port without ACS, the device itself
/// for a sibling function, or the bridge to a shared conventional bus, which
/// `shared` says (a port-only claim is blocked only so, port_only.rs).
#[derive(Clone, Copy)]
pub(super) struct Member {
    pub id: u64,
    pub at: (u8, u8, u8),
    pub hop: (u8, u8, u8),
    pub shared: bool,
}
