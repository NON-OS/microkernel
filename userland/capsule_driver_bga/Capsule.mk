# bga: the Bochs graphics adapter (QEMU stdvga, 1234:1111) as a gfx service,
# as Linux bochs.c drives it. PCI MMIO only: the DISPI registers in BAR 2 and
# the framebuffer in BAR 0. It keeps the firmware's mode when it can serve
# it, shares a shadow surface with the compositor and copies each present
# into VRAM. No DMA, no IRQ, no Pio.

CAPSULE_SLUG             := driver-bga
CAPSULE_HANDLE           := driver.bga0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_bga
CAPSULE_BIN_NAME         := driver_bga
CAPSULE_FEATURE          := nonos-capsule-driver-bga
CAPSULE_NAMESPACE        := systems.nonos.driver.bga0
CAPSULE_SERVICE_ENDPOINT := service:4240:driver.bga0
CAPSULE_REPLY_ENDPOINT   := reply:4241:endpoint.4294967326
# IPC|Memory|GraphicsSurfaceCreate|DeviceEnum|Driver|Mmio = 0x39018. Memory
# covers the shadow's mmap, GraphicsSurfaceCreate its register and share,
# Driver the PCI command write.
CAPSULE_REQUIRED_CAPS    := 0x39018
CAPSULE_KERNEL_MIRROR    := src/hardware/bga_capsule

include nonos-mk/capsule.mk
