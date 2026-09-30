# RTL8169 — Realtek 8168/8169 gigabit NIC. PCI MMIO + DMA, polled.
# Raw Ethernet frames only; network policy belongs to the net-stack
# capsule. Signing, certificate, and manifest rules come from the
# shared hybrid-signature capsule macro.

CAPSULE_SLUG             := driver-rtl8169
CAPSULE_HANDLE           := driver.rtl8169_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8169
CAPSULE_BIN_NAME         := driver_rtl8169
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8169
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8169_0
CAPSULE_SERVICE_ENDPOINT := service:4214:driver.rtl8169_0
CAPSULE_REPLY_ENDPOINT   := reply:4215:endpoint.4294967310
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8039
# Crypto (0x20) is what the CryptoRandom syscall is gated on. The station address
# is drawn rather than read out of the IDR, and that draw fails closed, so
# without this the card never comes up. No Irq: the driver polls.
CAPSULE_REQUIRED_CAPS    := 0xB8039

include nonos-mk/capsule.mk
