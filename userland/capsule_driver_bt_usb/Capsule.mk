# bt_usb: Bluetooth LE host over USB. Finds a controller (class E0/01/01)
# through driver.xhci0, brings it up with standard HCI commands only, finds
# a HID keyboard or mouse with a passive scan, pairs with LE Secure
# Connections and posts its input as driver.usb_hid0 posts a USB one.
# Needs B-43 in capsule_driver_xhci: an interrupt endpoint beside the bulk
# pair on one slot, interrupt reads up to 64 octets, OP_BULK_IN_POLL.

CAPSULE_SLUG             := driver-bt-usb
CAPSULE_HANDLE           := driver.bt_usb0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_bt_usb
CAPSULE_BIN_NAME         := driver_bt_usb
CAPSULE_FEATURE          := nonos-capsule-driver-bt-usb
CAPSULE_NAMESPACE        := systems.nonos.driver.bt_usb0
CAPSULE_SERVICE_ENDPOINT := service:4236:driver.bt_usb0
CAPSULE_REPLY_ENDPOINT   := reply:4237:endpoint.4294967324
# IPC | Memory | Crypto | Debug = 0x08 | 0x10 | 0x20 | 0x200000 = 0x200038
# Crypto is what the CryptoRandom syscall is gated on: pairing's key pair
# and nonces are drawn from it, and the capsule stops if a draw fails. No
# Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above xHCI.
CAPSULE_REQUIRED_CAPS    := 0x200038

include nonos-mk/capsule.mk
# Builds and is proven on the host, but has no owner certificate, no kernel
# embed and no spawn yet, and needs B-43 applied to driver.xhci0 first.
CAPSULE_NOT_IN_IMAGE     := 1
