# tg3: Broadcom 57765-class GbE (BCM57761, 57762, 57765, 57766, 57781,
# 57782, 57785, 57786, 57791, 57795). PCI MMIO + DMA, polled, one standard
# receive ring, its return ring and one send ring (three DMA grants).
# Frame-level transport over IPC; no socket or routing policy. `Network` is
# absent: that authority belongs to the stack capsules above this driver
# (net.core, net.l2). Crypto draws the per-boot station address. Wire shape
# matches `driver-e1000` so one net-stack client drives either backend.

CAPSULE_SLUG             := driver-tg3
CAPSULE_HANDLE           := driver.tg3_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_tg3
CAPSULE_BIN_NAME         := driver_tg3
CAPSULE_FEATURE          := nonos-capsule-driver-tg3
CAPSULE_NAMESPACE        := systems.nonos.driver.tg3_0
CAPSULE_SERVICE_ENDPOINT := service:4506:driver.tg3_0
CAPSULE_REPLY_ENDPOINT   := reply:4507:endpoint.4294967394
# The NIC mask, 0xB8038, each bit for one thing the driver does:
#   IPC (1<<3) serves NNET; Memory (1<<4) backs the heap; Crypto (1<<5)
#   draws the station address; DeviceEnum (1<<15) lists the function;
#   Driver (1<<16) claims it and writes its configuration words; Mmio
#   (1<<17) maps BAR0; Dma (1<<19) takes the three grants.
# No Irq: the part is polled. No Network, Pio, filesystem or admin.
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the bring-up and link lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
# Builds and is proven on the host, but is not yet embedded or spawned:
# scripts/hardware_support_evidence.py reports it as not in the image.
CAPSULE_NOT_IN_IMAGE     := 1

include nonos-mk/capsule.mk
