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
    /// Grant paths inside the claim now, each holding it until its grant is recorded (L66).
    pub(crate) inflight: u32,
    /// Releases under way, a `give_back`'s or an exit's: the entry goes with the last of them
    /// and the last path inside it, so no newer claim is made while one tears it down (L75).
    pub(crate) leavers: u32,
}

impl Claim {
    /// A claim as its call first enters it: nothing inside, nothing leaving.
    pub(crate) const fn making(pid: u32, device_id: u64, epoch: u64) -> Self {
        Self { pid, device_id, epoch, stage: Stage::Making, inflight: 0, leavers: 0 }
    }
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
    /// Being made when its holder exited: it keeps other claimants out until
    /// the call making it takes it back (L66).
    Orphaned,
    /// Its holder exited: nothing more is granted against it, a grant being recorded is undone
    /// by its own path, and the entry stays until those paths and every release are done (L75).
    Exiting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimError {
    UnknownDevice,
    AlreadyClaimed,
    NotHolder,
    NotClaimed,
    /// A remapping unit is in service and would not take the device.
    Unconfined,
    OutOfScope, // the claimant's scope row does not name the device
    /// the word for a device being worked on; the lifecycle says AlreadyClaimed or Resetting.
    Busy,
    /// Another capsule holds what this one reaches past the IOMMU, or its requester id.
    GroupHeld,
    Held, // held for the one capsule spawned to drive it
    /// Refused to every claimant, until the owner allows it or, for a bridge, a port claim (L58).
    Refused,
    /// Its last claim's wipe is under way, or its entry is going: ask again.
    Resetting,
    Firmware, // the firmware's flash controller, or a device inside its window
    Disabled, // the record is marked unusable (`DEVICE_FLAG_DISABLED`)
}
