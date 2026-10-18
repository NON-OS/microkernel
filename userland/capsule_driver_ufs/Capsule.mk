# ufs: UFS host controllers (UFSHCI 2.0 and later, and 1.x) on the listed
# ids, QEMU's ufs device among them. PCI MMIO + DMA,
# polled: four DMA grants (transfer list, task list, command descriptor,
# 64 KiB data). Each UFS logical unit is a unit on the block wire.
# Raw sectors only; who may reach them is the medium rule the kernel checks
# (StoreWrite). No Crypto, no Network.

CAPSULE_SLUG             := driver-ufs
CAPSULE_HANDLE           := driver.ufs0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ufs
CAPSULE_BIN_NAME         := driver_ufs
CAPSULE_FEATURE          := nonos-capsule-driver-ufs
CAPSULE_NAMESPACE        := systems.nonos.driver.ufs0
CAPSULE_SERVICE_ENDPOINT := service:4238:driver.ufs0
CAPSULE_REPLY_ENDPOINT   := reply:4239:endpoint.4294967325
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xB8018, the set driver.sdhci0
# holds. Driver covers MkPciConfigWrite. No Irq: completions are polled.
CAPSULE_REQUIRED_CAPS    := 0xB8018
CAPSULE_KERNEL_MIRROR    := src/hardware/ufs_capsule

include nonos-mk/capsule.mk
