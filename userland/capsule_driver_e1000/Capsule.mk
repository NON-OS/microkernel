# e1000: Intel 8254x gigabit NIC. PCI MMIO + DMA, polled, with
# separate RX and TX rings (four DMA grants total). Frame-level
# transport over IPC; no socket or routing policy. `Network` is
# absent: that authority belongs to the stack capsules above this
# driver (net.core, net.l2), the only ones the kernel lets send to
# it. The per-boot station address is drawn with CryptoRandom, which
# every capsule may call; no Crypto. Wire shape matches
# `driver-virtio-net` so one net-stack client drives either backend.

CAPSULE_SLUG             := driver-e1000
CAPSULE_HANDLE           := driver.e1000_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_e1000
CAPSULE_BIN_NAME         := driver_e1000
CAPSULE_FEATURE          := nonos-capsule-driver-e1000
CAPSULE_NAMESPACE        := systems.nonos.driver.e1000_0
CAPSULE_SERVICE_ENDPOINT := service:4210:driver.e1000_0
CAPSULE_REPLY_ENDPOINT   := reply:4211:endpoint.4294967308
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xB8018. No Irq: the driver polls
# and binds no line.
# No Crypto: the station address is drawn with CryptoRandom, which every
# capsule may call, rather than read out of the EEPROM, and that draw fails
# closed, so without entropy the card has no address to transmit under.
CAPSULE_REQUIRED_CAPS    := 0xB8018
# Debug, granted only by a build that compiles `capsule-serial-debug` (the
# development images): the kernel mirror folds it in through
# serial_debug_cap(), so the driver's own lines reach the console there and
# nowhere else.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/e1000_capsule

include nonos-mk/capsule.mk
