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

#[derive(Debug, Clone, Copy)]
pub struct Claim {
    pub pid: u32,
    pub device_id: u64,
    pub epoch: u64,
    pub(crate) stage: Stage,
    /// Grant paths inside the claim now, each holding it until its grant is
    /// recorded (L66).
    pub(crate) inflight: u32,
}

/// Where a claim is in its life. Only a ready one is granted against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stage {
    /// Being made: it keeps other claimants out, and only the call making it
    /// settles it, since it may still be refused (L65).
    Making,
    Ready,
    /// Being released: nothing more is granted, and the release waits out
    /// the grant paths already inside it before it drains (L66).
    Leaving,
    /// Being made when its holder exited: it still keeps other claimants out,
    /// until the call making it takes it back (L66).
    Orphaned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimError {
    UnknownDevice,
    AlreadyClaimed,
    NotHolder,
    NotClaimed,
    /// A remapping unit is in service and would not take the device.
    Unconfined,
    /// Held for the one capsule spawned to drive it.
    Held,
    /// Refused to every claimant: until the decision allows it, or
    /// for a bridge, until a claim made for ports exists (L58).
    Refused,
}
