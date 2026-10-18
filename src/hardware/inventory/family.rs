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
    /// virtio-scsi (1af4:1048, 1af4:1004): QEMU's virtio-scsi-pci.
    StorageVirtioScsi,
    /// BusLogic MultiMaster (104b:1040): VirtualBox's and VMware's BusLogic.
    StorageBuslogic,
    /// LSI Fusion-MPT (1000:0030, 1000:0054): VMware's and VirtualBox's
    /// LSI Logic Parallel and SAS, QEMU's mptsas1068.
    StorageMpt,
    /// VMware PVSCSI (15ad:07c0): VMware's paravirtual SCSI, QEMU's pvscsi.
    StorageVmwarePvscsi,
    /// An Intel VMD: a PCI domain hiding NVMe and SATA controllers behind
    /// it (`vmd`). Listed, never spawned for.
    StorageVmd,
    /// An Intel eMMC host (`emmc`): the soldered disk of Atom, Celeron and
    /// Pentium Silver laptops. Served by the SD host controller capsule.
    StorageEmmc,
    /// Any other SD host (`sd_host`): a card reader, or an embedded slot
    /// the eMMC list does not name; and an SD host the ACPI capsule
    /// published with no PCI function (`platform_sd`). Served by the same
    /// capsule.
    StorageSdhci,
    /// A UFS host controller, class 01h subclass 09h: driver.ufs0's.
    StorageUfs,
    /// Intel PIIX3/PIIX4 IDE (8086:7010, 8086:7111): QEMU's pc machine,
    /// VirtualBox's default, Hyper-V Generation 1.
    StorageIde,
    NetworkVirtio,
    /// The 8254x parts the e1000 capsule takes.
    NetworkE1000,
    /// 82574, 82583 and the PCH I217/I218/I219: the e1000e capsule's.
    NetworkE1000e,
    /// 82575 to I211: no capsule yet.
    NetworkIgb,
    /// I225 and I226: the igc capsule's.
    NetworkIgc,
    /// 82599, X540 and X550: the ixgbe capsule's.
    NetworkIxgbe,
    NetworkRtl8139,
    NetworkRtl8169,
    /// AMD PCnet (Am79C970A/Am79C973): VirtualBox's default NIC.
    NetworkPcnet,
    /// VMware vmxnet3: VMware's paravirtual NIC, also emulated by QEMU.
    NetworkVmxnet3,
    /// DEC 21140/21143: Hyper-V's legacy network adapter, QEMU's tulip.
    NetworkTulip,
    NetworkRtl8821ce,
    NetworkIwlwifi,
    /// A Realtek wireless part other than the RTL8821CE.
    NetworkRealtekWifi,
    DisplayGopFramebuffer,
    DisplayVirtioGpu,
    DisplayBga,
    /// SVGA II: VMware's adapter, VirtualBox's VMSVGA, QEMU's vmware-svga.
    DisplayVmwareSvga,
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
    /// virtio-input: QEMU's virtio tablet, keyboard and mouse.
    InputVirtio,
    AudioHda,
    /// VirtualBox's guest device (80ee:cafe): its mouse integration.
    InputVmmdev,
    /// Intel ICH AC'97 (8086:2415): QEMU's AC97 and VirtualBox's AC'97.
    AudioAc97,
    SerialI2c,
    /// An Intel SMBus host, class 0Ch subclass 05h: the i801 capsule's.
    SerialSmbus,
    SerialSpi,
    BridgePci,
    SystemPeripheral,
    Unknown,
}
