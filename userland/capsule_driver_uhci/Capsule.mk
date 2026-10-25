# uhci: a UHCI host controller, the USB 1.1 host of Intel and VIA chipsets
# and of QEMU's and VMware's virtual machines, served on driver.uhci0 with
# driver.xhci0's wire so the USB class drivers use it unchanged.

CAPSULE_SLUG             := driver-uhci
CAPSULE_HANDLE           := driver.uhci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_uhci
CAPSULE_BIN_NAME         := driver_uhci
CAPSULE_FEATURE          := nonos-capsule-driver-uhci
CAPSULE_NAMESPACE        := systems.nonos.driver.uhci0
CAPSULE_SERVICE_ENDPOINT := service:4508:driver.uhci0
CAPSULE_REPLY_ENDPOINT   := reply:4509:endpoint.4294967395
# IPC | Memory | DeviceEnum | Driver | Dma | Pio
#   = 0x08 | 0x10 | 0x8000 | 0x10000 | 0x80000 | 0x100000 = 0x198018
# Pio for the controller's registers, Dma for its schedule region. No Mmio
# or Irq: the controller is polled.
CAPSULE_REQUIRED_CAPS    := 0x198018
# Debug, granted only by a build that compiles `capsule-serial-debug`.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/uhci_capsule

include nonos-mk/capsule.mk
