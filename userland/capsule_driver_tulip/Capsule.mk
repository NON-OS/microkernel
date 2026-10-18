# tulip — DEC 21140/21143: Hyper-V's legacy network adapter (a 21140A) and
# QEMU's `tulip` (a 21143). MMIO BAR1 and DMA, polled (no Irq, no Pio).
# Frames only, on the NNET wire every wired card serves; the kernel holds
# the endpoint to net.core and net.l2.

CAPSULE_SLUG             := driver-tulip
CAPSULE_HANDLE           := driver.tulip0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_tulip
CAPSULE_BIN_NAME         := driver_tulip
CAPSULE_FEATURE          := nonos-capsule-driver-tulip
CAPSULE_NAMESPACE        := systems.nonos.driver.tulip0
CAPSULE_SERVICE_ENDPOINT := service:4286:driver.tulip0
CAPSULE_REPLY_ENDPOINT   := reply:4287:endpoint.4294967354
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8038. Crypto draws the
# station address. No Irq, Pio or Network.
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/tulip_capsule

include nonos-mk/capsule.mk
