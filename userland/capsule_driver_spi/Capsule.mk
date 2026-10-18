# spi — the Intel LPSS SPI host driver. Proves an LPSS function is an SPI
# host by its capabilities before writing to it, and serves raw transfers
# to one client. Never the firmware's flash controller: the broker refuses
# that claim to every capsule. Its endpoint is held to driver.spi_hid0
# alone.

CAPSULE_SLUG             := driver-spi
CAPSULE_HANDLE           := driver.spi0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_spi
CAPSULE_BIN_NAME         := driver_spi
CAPSULE_FEATURE          := nonos-capsule-driver-spi
CAPSULE_NAMESPACE        := systems.nonos.driver.spi0
CAPSULE_SERVICE_ENDPOINT := service:4528:driver.spi0
CAPSULE_REPLY_ENDPOINT   := reply:4529:endpoint.4294967485
# IPC, Memory, DeviceEnum, Driver and Mmio: the function is found, claimed
# and its BAR mapped; polled, so no Irq; no Dma and no Pio.
CAPSULE_REQUIRED_CAPS    := 0x38018
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [spi] lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/spi_capsule

include nonos-mk/capsule.mk
