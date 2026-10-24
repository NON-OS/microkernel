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

mod address;
mod bar;
mod bridge;
mod capability;
mod class_code;
mod device;
mod device_bars;
mod device_kind;
mod device_msi;
mod device_new;
mod device_query;
mod device_id;
mod header;
mod msi;
mod msi_message;
mod pcie;
mod power;

pub use address::PciAddress;
pub use bar::PciBar;
pub use bridge::BridgeInfo;
pub use capability::{PciCapability, PcieCapability};
pub use class_code::ClassCode;
pub use device::PciDevice;
pub use device_id::DeviceId;
pub use header::HeaderType;
pub use msi::{MsiInfo, MsixInfo};
pub use msi_message::MsiMessage;
pub use pcie::{PcieDeviceType, PcieInfo};
pub use power::PowerManagementInfo;
