# bt_usb: Bluetooth LE host over USB. Finds a controller (class E0/01/01)
# through driver.xhci0, loads the patch an RTL8821C needs (nonos_bt_rtl,
# the only vendor setup it does), brings it up with standard HCI, finds a
# HID keyboard or mouse with a passive scan, pairs with LE Secure
# Connections and posts its input as driver.usb_hid0 posts a USB one.
# Uses B-43 in capsule_driver_xhci: an interrupt endpoint beside the bulk
# pair on one slot, interrupt reads up to 64 octets, OP_BULK_IN_POLL.

CAPSULE_SLUG             := driver-bt-usb
CAPSULE_HANDLE           := driver.bt_usb0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_bt_usb
CAPSULE_BIN_NAME         := driver_bt_usb
CAPSULE_FEATURE          := nonos-capsule-driver-bt-usb
CAPSULE_NAMESPACE        := systems.nonos.driver.bt_usb0
CAPSULE_SERVICE_ENDPOINT := service:4342:driver.bt_usb0
CAPSULE_REPLY_ENDPOINT   := reply:4343:endpoint.4294967368
# IPC | Memory | Crypto | DeviceEnum | InputSource | Keyring
#   = 0x08 | 0x10 | 0x20 | 0x8000 | 0x200000 | 0x10000000 = 0x10208038
# Crypto is what the CryptoRandom syscall is gated on: pairing's key pair
# and nonces are drawn from it, and the capsule stops if a draw fails.
# InputSource posts the keys and motion, as driver.usb_hid0's 0x200000.
# Keyring lets it keep bonds and its own IRK there (lead unit L33).
# DeviceEnum, enumerate-only, is what MkInstallSource serves the boot
# medium's firmware packages (kinds 0x20 on) to: the MediaTek Bluetooth
# patch is one (its licence keeps it out of the capsule). No Driver, Mmio,
# Irq, Dma or Pio: this is a class capsule above xHCI and claims nothing.
CAPSULE_REQUIRED_CAPS    := 0x10208038
# Debug, for its bt-usb: lines, where the image lets capsules write them.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/bt_usb_capsule

include nonos-mk/capsule.mk
