# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
"""Each driver capsule's discover table, as check_driver_scope.py reads it."""

# service, a capsule file, and the items (consts or fns) naming its ids
DISCOVER = [
    ("driver.e1000_0", "e1000/src/constants/pci.rs", "E1000_DEVICE_IDS"),
    ("driver.e1000e_0", "e1000e/src/constants/ids.rs",
     "I82574 I82583 PCH_LPT PCH_SPT PCH_CNP PCH_TGP PCH_ADP PCH_MTP PCH_PTP"),
    ("driver.igb_0", "igb/src/facts/ids.rs", "IGB_PARTS"),
    ("driver.igc_0", "igc/src/constants/pci.rs", "IGC_DEVICE_IDS"),
    ("driver.ixgbe_0", "ixgbe/src/facts/ids.rs", "IXGBE_PARTS"),
    ("driver.rtl8169_0", "rtl8169/src/constants/pci.rs", "RTL8169_DEVICE_IDS"),
    ("driver.rtl8139_0", "rtl8139/src/constants/pci.rs", "RTL8139_DEVICE_ID"),
    ("driver.virtio_net0", "virtio_net/src/constants/pci.rs", "VIRTIO_NET_TRANSITIONAL VIRTIO_NET_MODERN"),
    ("driver.iwlwifi0", "iwlwifi/src/firmware/family.rs", "family_for_device"),
    ("driver.rtl8821ce0", "rtl8821ce/src/constants/mod.rs", "PCI_DEVICE_RTL8821CE"),
    ("driver.sdhci0", "sdhci/src/emmc/pci/ids.rs", "INTEL_EMMC"),
    ("driver.virtio_blk0", "virtio_blk/src/constants/pci.rs", "VIRTIO_BLK_TRANSITIONAL VIRTIO_BLK_MODERN"),
    ("driver.i2c_pci0", "i2c_pci/src/constants/device_info.rs", "device_info"),
    ("driver.virtio_rng", "virtio_rng/src/constants/pci.rs", "VIRTIO_RNG_TRANSITIONAL VIRTIO_RNG_MODERN"),
    ("driver.virtio_gpu0", "virtio_gpu/src/constants/pci.rs", "VIRTIO_GPU_MODERN"),
    ("driver.bga0", "bga/src/constants.rs", "DEVICE_BGA"),
    ("driver.ac97_0", "ac97/src/constants/pci.rs", "ICH_AC97_DEVICE_ID"),
    ("driver.buslogic0", "buslogic/src/constants/pci.rs", "MULTIMASTER_DEVICE_ID"),
    ("driver.mpt0", "mpt/src/constants/pci.rs", "LSI_53C1030_DEVICE_ID LSI_SAS1068_DEVICE_ID"),
    ("driver.pcnet0", "pcnet/src/constants/pci.rs", "PCNET_DEVICE_ID"),
    ("driver.piix_ide0", "piix_ide/src/constants/pci.rs", "PIIX3_IDE_DEVICE_ID PIIX4_IDE_DEVICE_ID"),
    ("driver.pvscsi0", "pvscsi/src/constants/pci.rs", "PVSCSI_DEVICE_ID"),
    ("driver.tulip0", "tulip/src/constants/pci.rs", "DEC_21140 DEC_21143"),
    ("driver.virtio_input0", "virtio_input/src/constants/pci.rs", "VIRTIO_INPUT_DEVICE_ID"),
    ("driver.virtio_scsi0", "virtio_scsi/src/constants/pci.rs",
     "VIRTIO_SCSI_MODERN_ID VIRTIO_SCSI_TRANSITIONAL_ID"),
    ("driver.vmmdev0", "vmmdev/src/constants/pci.rs", "VMMDEV_DEVICE_ID"),
    ("driver.vmware_svga0", "vmware_svga/src/constants/pci.rs", "SVGA2_DEVICE_ID VBOXSVGA_DEVICE_ID"),
    ("driver.vmxnet3_0", "vmxnet3/src/constants/pci.rs", "VMXNET3_DEVICE_ID"),
]
