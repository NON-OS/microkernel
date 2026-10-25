# netvsc — Hyper-V's synthetic network adapter, on the VMBus channel the
# VMBus core gives it (its record: 1414:4E45 on the virtual bus, the relid in
# irq_source). Three coherent DMA grants for the channel's rings, the
# receive buffer and the send buffer, polled (no Irq). NVSP and RNDIS on the
# channel; frames to net.core on the NNET wire. The drawn station address
# if the host carries it, else the host-assigned one, said on the console.

CAPSULE_SLUG             := driver-netvsc
CAPSULE_HANDLE           := driver.netvsc0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_netvsc
CAPSULE_BIN_NAME         := driver_netvsc
CAPSULE_FEATURE          := nonos-capsule-driver-netvsc
CAPSULE_NAMESPACE        := systems.nonos.driver.netvsc0
CAPSULE_SERVICE_ENDPOINT := service:4306:driver.netvsc0
CAPSULE_REPLY_ENDPOINT   := reply:4307:endpoint.4294967398
# IPC|Memory|Crypto|Driver|DeviceEnum|Dma = 0x98038. Crypto for the drawn
# station address and the probe's DHCP transaction id. Driver for the
# channel record's claim and its MkHv* calls. No Mmio or Pio: a channel has
# neither. No Irq: the driver polls the host's ring.
CAPSULE_REQUIRED_CAPS    := 0x98038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/netvsc_capsule

include nonos-mk/capsule.mk
