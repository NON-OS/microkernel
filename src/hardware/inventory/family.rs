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
    /// Pentium Silver laptops. Served by the AHCI capsule at present.
    StorageEmmc,
    /// Intel PIIX3/PIIX4 IDE (8086:7010, 8086:7111): QEMU's pc machine,
    /// VirtualBox's default, Hyper-V Generation 1.
    StorageIde,
    NetworkVirtio,
    NetworkE1000,
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
    SerialSpi,
    BridgePci,
    SystemPeripheral,
    Unknown,
}
