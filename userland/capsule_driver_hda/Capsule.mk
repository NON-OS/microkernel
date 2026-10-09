# Intel HDA: HD-Audio controller capsule. PCI MMIO + DMA, with INTx,
# else one MSI-X vector, else polling. Owns controller discovery,
# broker claim, BAR0 mapping, reset release, the CORB/RIRB verb rings,
# codec presence, one DAC-to-pin output path, and one output stream
# (BDL and PCM ring in DMA) that plays what audio.server sends.
# No capture stream.

CAPSULE_SLUG             := driver-hda
CAPSULE_HANDLE           := driver.hda0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_hda
CAPSULE_BIN_NAME         := driver_hda
CAPSULE_FEATURE          := nonos-capsule-driver-hda
CAPSULE_NAMESPACE        := systems.nonos.driver.hda0
CAPSULE_SERVICE_ENDPOINT := service:4218:driver.hda0
CAPSULE_REPLY_ENDPOINT   := reply:4219:endpoint.4294967312
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq|Dma = 0xF8018
CAPSULE_REQUIRED_CAPS    := 0xF8018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/hda_capsule
include nonos-mk/capsule.mk
