# igb: Intel 82575, 82576, 82580, I350, I210 and I211 GbE NICs on a copper
# port (QEMU's igb is an 82576). PCI MMIO + DMA, polled, one advanced RX
# queue and one advanced TX queue (four DMA grants total). Frame-level
# transport over IPC; no socket or routing policy. `Network` is absent:
# that authority belongs to the stack capsules above this driver (net.core,
# net.l2). The per-boot station address is drawn with CryptoRandom, which
# every capsule may call; no Crypto. Wire shape matches `driver-e1000` so
# one net-stack client drives either backend.

CAPSULE_SLUG             := driver-igb
CAPSULE_HANDLE           := driver.igb_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_igb
CAPSULE_BIN_NAME         := driver_igb
CAPSULE_FEATURE          := nonos-capsule-driver-igb
CAPSULE_NAMESPACE        := systems.nonos.driver.igb_0
CAPSULE_SERVICE_ENDPOINT := service:4242:driver.igb_0
CAPSULE_REPLY_ENDPOINT   := reply:4243:endpoint.4294967329
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xB8018, the set rtl8169
# holds. Driver covers MkPciConfigWrite, which sets Bus Master and Memory
# Space after the claim. No Irq: the driver polls and binds no line.
# No Crypto: the station address is drawn with CryptoRandom, which every
# capsule may call, rather than read out of the NVM, and that draw fails
# closed, so without entropy the card has no address to transmit under.
CAPSULE_REQUIRED_CAPS    := 0xB8018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(). Every igb: line goes
# out through mk_debug, which is gated on it, so without this bound the project's
# `log igb` would show nothing of where a bring-up stopped.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/igb_capsule

include nonos-mk/capsule.mk
