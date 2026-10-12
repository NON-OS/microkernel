# ohci: an OHCI host controller, the USB 1.1 host of AMD, SiS, NEC and
# Apple chipsets and of VirtualBox and QEMU, served on driver.ohci0 with
# driver.xhci0's wire so the USB class drivers use it unchanged.

CAPSULE_SLUG             := driver-ohci
CAPSULE_HANDLE           := driver.ohci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ohci
CAPSULE_BIN_NAME         := driver_ohci
CAPSULE_FEATURE          := nonos-capsule-driver-ohci
CAPSULE_NAMESPACE        := systems.nonos.driver.ohci0
CAPSULE_SERVICE_ENDPOINT := service:4510:driver.ohci0
CAPSULE_REPLY_ENDPOINT   := reply:4511:endpoint.4294967396
# IPC | Memory | DeviceEnum | Driver | Mmio | Dma
#   = 0x08 | 0x10 | 0x8000 | 0x10000 | 0x20000 | 0x80000 = 0xB8018
# Mmio for the controller's registers, Dma for its schedule region. No Pio
# or Irq: the controller is polled.
CAPSULE_REQUIRED_CAPS    := 0xB8018
# Debug, granted only by a build that compiles `capsule-serial-debug`.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/ohci_capsule

include nonos-mk/capsule.mk
