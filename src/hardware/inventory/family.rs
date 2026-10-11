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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareFamily {
    StorageNvme,
    StorageAhci,
    StorageUsbMsc,
    StorageVirtioBlk,
    /// An Intel VMD: a PCI domain hiding NVMe and SATA controllers behind
    /// it (`vmd`). Listed, never spawned for.
    StorageVmd,
    /// An Intel eMMC host (`emmc`): the soldered disk of Atom, Celeron and
    /// Pentium Silver laptops. Served by the SD host controller capsule.
    StorageEmmc,
    /// Any other SD host (`sd_host`): a card reader, or an embedded slot
    /// the eMMC list does not name. Served by the same capsule.
    StorageSdhci,
    /// A UFS host controller, class 01h subclass 09h: no capsule yet.
    StorageUfs,
    NetworkVirtio,
    /// The 8254x parts the e1000 capsule takes.
    NetworkE1000,
    /// 82574, 82583 and the PCH I217/I218/I219: the e1000e capsule's.
    NetworkE1000e,
    /// 82575 to I211: no capsule yet.
    NetworkIgb,
    /// I225 and I226: the igc capsule's.
    NetworkIgc,
    NetworkRtl8139,
    NetworkRtl8169,
    NetworkRtl8821ce,
    NetworkIwlwifi,
    /// A Realtek wireless part other than the RTL8821CE.
    NetworkRealtekWifi,
    DisplayGopFramebuffer,
    DisplayVirtioGpu,
    DisplayBga,
    DisplayNativeIntel,
    DisplayNativeAmd,
    DisplayNativeNvidia,
    UsbXhci,
    UsbEhci,
    UsbOhci,
    UsbUhci,
    InputPs2,
    InputUsbHid,
    InputI2cHid,
    AudioHda,
    SerialI2c,
    /// An Intel SMBus host, class 0Ch subclass 05h: the i801 capsule's.
    SerialSmbus,
    SerialSpi,
    BridgePci,
    SystemPeripheral,
    Unknown,
}
