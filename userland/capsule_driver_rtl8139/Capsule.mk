# RTL8139 — Realtek 8139 Fast Ethernet NIC. PCI PIO + DMA, polled.
# Frame-level transport only: no socket, routing, ARP, or IP policy.
# Signing, certificate, manifest, and trust-anchor flow are inherited
# from nonos-mk/capsule.mk.

CAPSULE_SLUG             := driver-rtl8139
CAPSULE_HANDLE           := driver.rtl8139_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8139
CAPSULE_BIN_NAME         := driver_rtl8139
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8139
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8139_0
CAPSULE_SERVICE_ENDPOINT := service:4212:driver.rtl8139_0
CAPSULE_REPLY_ENDPOINT   := reply:4213:endpoint.4294967309
# IPC|Memory|Crypto|Driver|DeviceEnum|Dma|Pio = 0x198038
# Crypto (0x20) is what the CryptoRandom syscall is gated on. The station address
# is drawn rather than read out of the IDR, and that draw fails closed, so
# without this the card never comes up. No Irq: the driver polls.
CAPSULE_REQUIRED_CAPS    := 0x198038
# Debug, granted only by a build that compiles `capsule-serial-debug` (the
# development images): the kernel mirror folds it in through
# serial_debug_cap(), so the driver's own lines, its bring-up failure
# among them, reach the console there and nowhere else.
CAPSULE_OPTIONAL_CAPS    := 0x100

include nonos-mk/capsule.mk
