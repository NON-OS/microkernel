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

//! Offsets and limits from the PCI and virtio 1.0 specifications.

/// PCI status register, and its bit saying a capability list exists.
pub const PCI_STATUS: usize = 0x06;
pub const STATUS_CAP_LIST: u16 = 1 << 4;
/// The capability list head.
pub const CAP_POINTER: usize = 0x34;
/// Capabilities sit after the 64-byte standard header. A pointer below this
/// ends the walk; zero is the ordinary terminator.
pub const CAP_FIRST: u8 = 0x40;
/// The walk visits at most this many capabilities: 192 bytes of capability
/// area hold no more than 48 four-byte entries.
pub const MAX_CAPS: usize = 48;

pub const CAP_ID_VENDOR: u8 = 0x09;
pub const CAP_ID_MSIX: u8 = 0x11;

/// Fields of struct virtio_pci_cap, from the capability's first byte.
pub const CAP_NEXT: usize = 1;
pub const CAP_LEN: usize = 2;
pub const CAP_CFG_TYPE: usize = 3;
pub const CAP_BAR: usize = 4;
pub const CAP_OFFSET: usize = 8;
pub const CAP_LENGTH: usize = 12;
/// struct virtio_pci_notify_cap appends the multiplier.
pub const CAP_NOTIFY_MULTIPLIER: usize = 16;
pub const VIRTIO_CAP_MIN_LEN: u8 = 16;
pub const NOTIFY_CAP_MIN_LEN: u8 = 20;

pub const CFG_COMMON: u8 = 1;
pub const CFG_NOTIFY: u8 = 2;
pub const CFG_ISR: u8 = 3;
pub const CFG_DEVICE: u8 = 4;
/// The config-space access window. The drivers never use it.
pub const CFG_PCI: u8 = 5;

/// sizeof(struct virtio_pci_common_cfg): every register through queue_device.
pub const COMMON_CFG_LEN: u32 = 0x38;

/// MSI-X capability: message control, then the table and PBA offset/BIR words.
pub const MSIX_CONTROL: usize = 2;
pub const MSIX_TABLE: usize = 4;
pub const MSIX_PBA: usize = 8;
pub const MSIX_CAP_LEN: usize = 12;
