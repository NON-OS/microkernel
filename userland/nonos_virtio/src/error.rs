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

//! Why a modern bring-up stopped: a value the proofs compare, and a message
//! the driver hands to its bring-up loop.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtioError {
    /// The broker refused a config-space read.
    ConfigRead,
    /// No usable common configuration capability.
    NoCommonCfg,
    /// No usable notify capability.
    NoNotifyCfg,
    /// No usable ISR capability.
    NoIsrCfg,
    /// No usable device configuration capability.
    NoDeviceCfg,
    /// The broker refused to map a region.
    MapRefused,
    /// The broker mapped less of a region than the driver reads.
    MapShort,
    /// The device did not read back zero after the reset.
    ResetTimeout,
    /// The device does not offer VIRTIO_F_VERSION_1.
    NoVersion1,
    /// The device cleared FEATURES_OK.
    FeaturesRejected,
    /// The device has no such queue: its size reads zero.
    QueueMissing,
    /// The device's largest queue is smaller than the ring layout needs.
    QueueTooSmall,
    /// A ring address breaks the alignment the specification requires.
    RingUnaligned,
    /// The queue's notify address is outside the mapped notify region.
    NotifyOutOfRange,
    /// The device did not read queue_enable back as set.
    QueueNotEnabled,
    /// The device configuration region is shorter than the fields read.
    DeviceCfgShort,
    /// The device configuration kept changing under the read.
    DeviceCfgUnstable,
}

impl VirtioError {
    pub const fn message(self) -> &'static str {
        match self {
            Self::ConfigRead => "virtio: config space read refused",
            Self::NoCommonCfg => "virtio: no usable common cfg capability",
            Self::NoNotifyCfg => "virtio: no usable notify capability",
            Self::NoIsrCfg => "virtio: no usable isr capability",
            Self::NoDeviceCfg => "virtio: no usable device cfg capability",
            Self::MapRefused => "virtio: modern region map refused",
            Self::MapShort => "virtio: modern region mapped short",
            Self::ResetTimeout => "virtio: device did not reset",
            Self::NoVersion1 => "virtio: VERSION_1 not offered",
            Self::FeaturesRejected => "virtio: features-ok rejected",
            Self::QueueMissing => "virtio: queue missing",
            Self::QueueTooSmall => "virtio: queue smaller than the ring layout",
            Self::RingUnaligned => "virtio: ring address unaligned",
            Self::NotifyOutOfRange => "virtio: notify address outside the notify region",
            Self::QueueNotEnabled => "virtio: queue did not enable",
            Self::DeviceCfgShort => "virtio: device cfg region too short",
            Self::DeviceCfgUnstable => "virtio: device cfg kept changing",
        }
    }
}
