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

//! What the census counts.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Census {
    /// Live processes counted (not new, not exited).
    pub running: u32,
    /// Of those, admitted by the gate.
    pub admitted: u32,
    /// Admitted under a proof against the vendor's tree (authority 0).
    pub vendor: u32,
    /// Admitted under a proof against a root enrolled on this machine.
    pub enrolled: u32,
    /// Admitted on a publisher's signature alone.
    pub signed_only: u32,
    /// Capability-free processes outside the registry: hosted guests.
    pub sandboxed: u32,
    /// init.
    pub kernel: u32,
    /// Too young to have been recorded yet; neither passed nor failed.
    pub starting: u32,
    /// Holding a capability with no admission record. Must be zero.
    pub unadmitted: u32,
    /// Admitted capsules that hold Network.
    pub network: u32,
    /// The first unadmitted pid, for the screen to name.
    pub first_unadmitted: Option<u32>,
}
