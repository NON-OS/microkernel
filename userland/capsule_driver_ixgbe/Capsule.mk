# ixgbe: Intel 82599 (X520), X540 and X550 10GbE NICs. PCI MMIO + DMA,
# polled, one advanced RX queue and one advanced TX queue (four DMA grants
# total). Frame-level transport over IPC; no socket or routing policy.
# `Network` is absent: that authority belongs to the stack capsules above
# this driver (net.core, net.l2). Crypto draws the per-boot station address.
# Wire shape matches `driver-e1000` so one net-stack client drives either
# backend.

CAPSULE_SLUG             := driver-ixgbe
CAPSULE_HANDLE           := driver.ixgbe_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ixgbe
CAPSULE_BIN_NAME         := driver_ixgbe
CAPSULE_FEATURE          := nonos-capsule-driver-ixgbe
CAPSULE_NAMESPACE        := systems.nonos.driver.ixgbe_0
CAPSULE_SERVICE_ENDPOINT := service:4248:driver.ixgbe_0
CAPSULE_REPLY_ENDPOINT   := reply:4249:endpoint.4294967393
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8038, the NIC mask.
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the bring-up and link lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/ixgbe_capsule

include nonos-mk/capsule.mk
