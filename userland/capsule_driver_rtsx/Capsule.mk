# rtsx: Realtek PCIe SD card reader capsule (RTS5227, RTS522A). It owns the
# broker claim, the register BAR, two DMA buffers below 4 GiB, the chip
# bring-up and the SD card protocol; it polls the reader and takes no IRQ.

CAPSULE_SLUG             := driver-rtsx
CAPSULE_HANDLE           := driver.rtsx0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtsx
CAPSULE_BIN_NAME         := driver_rtsx
CAPSULE_FEATURE          := nonos-capsule-driver-rtsx
CAPSULE_NAMESPACE        := systems.nonos.driver.rtsx0
CAPSULE_SERVICE_ENDPOINT := service:4290:driver.rtsx0
CAPSULE_REPLY_ENDPOINT   := reply:4291:endpoint.4294967380
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xB8018: no Irq, the driver polls.
CAPSULE_REQUIRED_CAPS    := 0xB8018
# Debug, granted only by a build that compiles `capsule-serial-debug`. A
# laptop's reader is debugged from the "rtsx:" lines alone.
CAPSULE_OPTIONAL_CAPS    := 0x100

include nonos-mk/capsule.mk
# Half done (README): no owner certificate, no kernel embed, no spawn.
CAPSULE_NOT_IN_IMAGE     := 1
