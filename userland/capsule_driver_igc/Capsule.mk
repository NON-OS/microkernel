# igc: Intel I225/I226 2.5 GbE NIC. PCI MMIO + DMA, polled, one advanced
# RX queue and one advanced TX queue (four DMA grants total). Frame-level
# transport over IPC; no socket or routing policy. `Network` is absent:
# that authority belongs to the stack capsules above this driver (net.core,
# net.l2). Crypto draws the per-boot station address. Wire shape matches
# `driver-e1000` so one net-stack client drives either backend.

CAPSULE_SLUG             := driver-igc
CAPSULE_HANDLE           := driver.igc_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_igc
CAPSULE_BIN_NAME         := driver_igc
CAPSULE_FEATURE          := nonos-capsule-driver-igc
CAPSULE_NAMESPACE        := systems.nonos.driver.igc_0
CAPSULE_SERVICE_ENDPOINT := service:4272:driver.igc_0
CAPSULE_REPLY_ENDPOINT   := reply:4273:endpoint.4294967391
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8038, the set rtl8169
# holds. Driver covers MkPciConfigWrite, which sets Bus Master and Memory
# Space after the claim. No Irq: the driver polls and binds no line.
# Crypto (0x20) is what the CryptoRandom syscall is gated on. The station
# address is drawn rather than read out of the NVM, and that draw fails
# closed, so without this the card has no address to transmit under.
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(). Every igc: line goes
# out through mk_debug, which is gated on it, so without this bound the owner's
# `log igc` would show nothing of where a bring-up stopped.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/igc_capsule

include nonos-mk/capsule.mk
# Builds and is proven on the host, but has no owner certificate, no kernel
# embed and no spawn yet: scripts/hardware_support_evidence.py reports it as
# not in the image instead of as supported hardware. Remove when it ships.
CAPSULE_NOT_IN_IMAGE     := 1
