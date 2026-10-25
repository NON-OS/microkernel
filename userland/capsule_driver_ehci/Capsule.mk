# ehci: an EHCI host controller, the USB 2.0 high-speed host of chipsets
# before xHCI and of QEMU, VirtualBox and VMware, served on driver.ehci0 with
# driver.xhci0's wire; full- and low-speed devices go to its companions.

CAPSULE_SLUG             := driver-ehci
CAPSULE_HANDLE           := driver.ehci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ehci
CAPSULE_BIN_NAME         := driver_ehci
CAPSULE_FEATURE          := nonos-capsule-driver-ehci
CAPSULE_NAMESPACE        := systems.nonos.driver.ehci0
CAPSULE_SERVICE_ENDPOINT := service:4512:driver.ehci0
CAPSULE_REPLY_ENDPOINT   := reply:4513:endpoint.4294967397
# IPC | Memory | DeviceEnum | Driver | Mmio | Dma
#   = 0x08 | 0x10 | 0x8000 | 0x10000 | 0x20000 | 0x80000 = 0xB8018
# Mmio for the controller's registers, Dma for its schedule region. No Pio
# or Irq: the controller is polled.
CAPSULE_REQUIRED_CAPS    := 0xB8018
# Debug, granted only by a build that compiles `capsule-serial-debug`.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/ehci_capsule

include nonos-mk/capsule.mk
