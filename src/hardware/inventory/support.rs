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

use super::family::HardwareFamily::{self, *};
use super::state::SupportState::{self, *};

pub fn support_state(family: HardwareFamily) -> SupportState {
    match family {
        StorageVirtioBlk | StorageUsbMsc | StorageAhci | StorageNvme => DataPath,
        StorageEmmc | StorageSdhci | StorageUfs => DataPath,
        StorageIde | StorageVirtioScsi | StorageVmwarePvscsi | StorageBuslogic | StorageMpt => {
            DataPath
        }
        NetworkVirtio | NetworkE1000 | NetworkRtl8139 | NetworkRtl8169 => DataPath,
        NetworkE1000e | NetworkIgb | NetworkIgc | NetworkIxgbe => DataPath,
        NetworkRtl8821ce => DataPath,
        NetworkPcnet => DataPath,
        NetworkVmxnet3 => DataPath,
        NetworkTulip => DataPath,
        DisplayGopFramebuffer | DisplayVirtioGpu | DisplayBga => DataPath,
        DisplayVmwareSvga | DisplayNativeIntel => DataPath,
        UsbXhci | UsbEhci | UsbOhci | UsbUhci | InputPs2 | InputUsbHid => DataPath,
        InputVirtio => DataPath,
        AudioAc97 => DataPath,
        InputVmmdev => DataPath,
        NetworkIwlwifi | AudioHda | InputI2cHid | SerialI2c => ControllerStatus,
        SerialSmbus => DataPath,
        DisplayNativeAmd | DisplayNativeNvidia => EnumerateOnly,
        StorageVmd | NetworkRealtekWifi => EnumerateOnly,
        SerialSpi | BridgePci | SystemPeripheral | Unknown => EnumerateOnly,
    }
}
