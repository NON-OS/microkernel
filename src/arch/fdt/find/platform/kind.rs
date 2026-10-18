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

//! The device-tree devices a capsule may be handed, by `compatible`.
//!
//! An allowlist, not every node with a `reg`: the GIC, the timer and the
//! firmware's mailbox are nodes too, and a capsule granted one of those holds
//! the machine. Each entry is a programming interface a capsule speaks, which
//! the broker states the way a PCI function's class code would.

/// Which interface a platform device presents. The value is the record's
/// `device` field, part of the broker ABI.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatformKind {
    /// An SD Host Controller Specification register set: the Pi 4's eMMC2.
    Sdhci = 1,
    /// An xHCI register set (bindings' `generic-xhci`).
    Xhci = 2,
    /// An AHCI register set (bindings' `generic-ahci`).
    Ahci = 3,
    /// Broadcom GENET v5, the Pi 4's Ethernet MAC.
    GenetV5 = 4,
}

pub(super) const NEEDLES: &[&[u8]] =
    &[b"brcm,bcm2711-emmc2", b"generic-xhci", b"generic-ahci", b"brcm,bcm2711-genet-v5"];

pub(super) const KINDS: [PlatformKind; 4] =
    [PlatformKind::Sdhci, PlatformKind::Xhci, PlatformKind::Ahci, PlatformKind::GenetV5];
