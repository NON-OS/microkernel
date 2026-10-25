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

//! The kernel's driver-grant surface as it stands (A10): each call, the cap
//! the contract layer checks (not Admin, since K6), what the broker checks of
//! the hold, and what it hands out. The cap column is tested against the
//! real cap table; the rest is read, sites in HANDOFF/contracts/driver-*.

use crate::capabilities::Capability;
use crate::syscall::numbers::SyscallNumber as N;

/// What the broker checks of the caller's hold before it acts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hold {
    /// Nothing held: the call lists, or makes a claim.
    None,
    /// The claim's pid, not its epoch.
    Claim,
    /// The claim's pid and its epoch.
    ClaimEpoch,
    /// The grant's holder pid only.
    Grant,
    /// The grant's holder, and the claim's pid and epoch still live.
    GrantLive,
}

/// What a call hands out or takes back.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gives {
    Records,
    Claim,
    Teardown,
    ConfigSpace,
    MmioPages,
    Vectors,
    DmaFrames,
    Ports,
    Release,
}

pub struct Row(pub N, pub Capability, pub Hold, pub Gives);

pub const SURFACE: &[Row] = &[
    Row(N::MkDeviceList, Capability::DeviceEnum, Hold::None, Gives::Records),
    Row(N::MkDeviceClaim, Capability::Driver, Hold::None, Gives::Claim),
    Row(N::MkDeviceRelease, Capability::Driver, Hold::Claim, Gives::Teardown),
    Row(N::MkPciConfigRead, Capability::Driver, Hold::ClaimEpoch, Gives::ConfigSpace),
    Row(N::MkPciConfigWrite, Capability::Driver, Hold::ClaimEpoch, Gives::ConfigSpace),
    Row(N::MkMmioMap, Capability::Mmio, Hold::ClaimEpoch, Gives::MmioPages),
    Row(N::MkMmioUnmap, Capability::Mmio, Hold::Grant, Gives::Release),
    Row(N::MkIrqBind, Capability::Irq, Hold::ClaimEpoch, Gives::Vectors),
    Row(N::MkIrqUnbind, Capability::Irq, Hold::Grant, Gives::Release),
    Row(N::MkIrqAck, Capability::Irq, Hold::Grant, Gives::Vectors),
    Row(N::MkIrqPoll, Capability::Irq, Hold::Grant, Gives::Vectors),
    Row(N::MkIrqWait, Capability::Irq, Hold::Grant, Gives::Vectors),
    Row(N::MkDmaMap, Capability::Dma, Hold::ClaimEpoch, Gives::DmaFrames),
    Row(N::MkDmaUnmap, Capability::Dma, Hold::Grant, Gives::Release),
    Row(N::MkPioGrant, Capability::Pio, Hold::ClaimEpoch, Gives::Ports),
    Row(N::MkPioRead, Capability::Pio, Hold::GrantLive, Gives::Ports),
    Row(N::MkPioWrite, Capability::Pio, Hold::GrantLive, Gives::Ports),
    Row(N::MkPioRelease, Capability::Pio, Hold::Grant, Gives::Release),
];
