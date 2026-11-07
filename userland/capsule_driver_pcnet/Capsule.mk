# PCnet: AMD Am79C970A / Am79C973 (1022:2000), VirtualBox's default NIC
# and QEMU's `pcnet`. PCI MMIO (BAR1) + DMA, polled, with one coherent
# control page for the init block and both rings, and a receive and a
# transmit pool, all below 4 GiB. Raw Ethernet frames only; network policy
# belongs to the stack capsules (net.core, net.l2), the only ones the kernel
# lets send to it.

CAPSULE_SLUG             := driver-pcnet
CAPSULE_HANDLE           := driver.pcnet0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_pcnet
CAPSULE_BIN_NAME         := driver_pcnet
CAPSULE_FEATURE          := nonos-capsule-driver-pcnet
CAPSULE_NAMESPACE        := systems.nonos.driver.pcnet0
CAPSULE_SERVICE_ENDPOINT := service:4274:driver.pcnet0
CAPSULE_REPLY_ENDPOINT   := reply:4275:endpoint.4294967348
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xB8018. No Irq: the driver polls.
# No Pio: BAR1 is the register file's memory-mapped twin. No Crypto: the
# station address is drawn with CryptoRandom, which every capsule may call;
# the draw fails closed, so without entropy the card never comes up.
CAPSULE_REQUIRED_CAPS    := 0xB8018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(), so `log pcnet` shows
# where a bring-up stopped.
CAPSULE_OPTIONAL_CAPS    := 0x100

include nonos-mk/capsule.mk
