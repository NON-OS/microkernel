# smbus: the SMBus host controller of every Intel ICH and PCH,
# reached through its I/O BAR. It serves SMBus transfers on driver.smbus0
# and owns no device on the bus: what sits there (SPD EEPROMs, sensors, an
# SMBus touchpad) is for drivers above it.

CAPSULE_SLUG             := driver-smbus
CAPSULE_HANDLE           := driver.smbus0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_smbus
CAPSULE_BIN_NAME         := driver_smbus
CAPSULE_FEATURE          := nonos-capsule-driver-smbus
CAPSULE_NAMESPACE        := systems.nonos.driver.smbus0
CAPSULE_SERVICE_ENDPOINT := service:4244:driver.smbus0
CAPSULE_REPLY_ENDPOINT   := reply:4245:endpoint.4294967328
# IPC | Memory | DeviceEnum | Driver | Pio
#   = 0x08 | 0x10 | 0x8000 | 0x10000 | 0x100000 = 0x118018
# Pio for the host's ports. No Mmio, Irq or Dma: the host is polled and
# moves every byte through its registers.
CAPSULE_REQUIRED_CAPS    := 0x118018
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the bring-up and SPD lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/smbus_capsule

include nonos-mk/capsule.mk
