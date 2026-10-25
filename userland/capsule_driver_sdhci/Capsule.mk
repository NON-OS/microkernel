# SDHCI: SD Host Controller capsule. PCI MMIO + DMA; the INTx pin is
# turned off and every command completion is polled. Owns host discovery,
# broker claim, slot window mapping, ADMA2 descriptor and data buffers, the
# eMMC card bring-up, and the block wire (geometry, read, write, flush).

CAPSULE_SLUG             := driver-sdhci
CAPSULE_HANDLE           := driver.sdhci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_sdhci
CAPSULE_BIN_NAME         := driver_sdhci
CAPSULE_FEATURE          := nonos-capsule-driver-sdhci
CAPSULE_NAMESPACE        := systems.nonos.driver.sdhci0
CAPSULE_SERVICE_ENDPOINT := service:4260:driver.sdhci0
CAPSULE_REPLY_ENDPOINT   := reply:4261:endpoint.4294967324
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xb8018. No Irq: completions are
# polled and the INTx pin is off, so no MkIrq* call is made. A move to
# interrupts takes the bit back.
CAPSULE_REQUIRED_CAPS    := 0xb8018
CAPSULE_KERNEL_MIRROR    := src/hardware/sdhci_capsule

include nonos-mk/capsule.mk
