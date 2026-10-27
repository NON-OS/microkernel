# MT7921E/MT7922: MediaTek PCIe Wi-Fi (Linux v6.12 mt76/mt7921). Hardware
# authority is brokered DeviceClaim/MMIO/DMA only, polled; Crypto draws the
# station address. 802.11 and WPA2/WPA3 run in the linked shared wifi core;
# the net stack stays in the upper network capsules. The card's firmware is
# not in this capsule: the loader reads the pinned package from the boot
# medium and the driver reads it through MkInstallSource (DeviceEnum) and
# checks each file's sha256. Signing, certificate, and manifest rules come
# from the shared capsule macro.

CAPSULE_SLUG             := driver-mt7921
CAPSULE_HANDLE           := driver.mt7921e0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_mt7921
CAPSULE_BIN_NAME         := driver_mt7921
CAPSULE_FEATURE          := nonos-capsule-driver-mt7921
CAPSULE_NAMESPACE        := systems.nonos.driver.mt7921e0
CAPSULE_SERVICE_ENDPOINT := service:4264:driver.mt7921e0
CAPSULE_REPLY_ENDPOINT   := reply:4265:endpoint.4294967372
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8038
# No Irq: the driver polls its rings and makes no MkIrq* call.
# Crypto (0x20) is what CryptoRandom is gated on: the station address is
# drawn, and without it the radio stays down rather than use the card's own.
# DeviceEnum (0x8000) also admits MkInstallSource, the firmware package's read.
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/mt7921_capsule

include nonos-mk/capsule.mk
