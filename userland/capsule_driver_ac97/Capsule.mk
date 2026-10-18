# ac97 — the Intel ICH AC'97 controller (8086:2415): QEMU's AC97 and
# VirtualBox's AC'97. Port I/O for the mixer and bus master windows, one
# coherent DMA grant for the list and the ring, polled (no Irq). Serves
# audio.server the PCM sink wire driver.hda0 serves; the kernel holds the
# endpoint to the audio server alone.

CAPSULE_SLUG             := driver-ac97
CAPSULE_HANDLE           := driver.ac97_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ac97
CAPSULE_BIN_NAME         := driver_ac97
CAPSULE_FEATURE          := nonos-capsule-driver-ac97
CAPSULE_NAMESPACE        := systems.nonos.driver.ac97_0
CAPSULE_SERVICE_ENDPOINT := service:4282:driver.ac97_0
CAPSULE_REPLY_ENDPOINT   := reply:4283:endpoint.4294967352
# IPC|Memory|Driver|DeviceEnum|Dma|Pio = 0x198018. No Mmio: the ICH's
# windows are I/O ports. No Irq: the driver polls the engine's position.
CAPSULE_REQUIRED_CAPS    := 0x198018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/ac97_capsule

include nonos-mk/capsule.mk
