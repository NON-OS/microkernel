# vmware-svga — the SVGA II display adapter (15ad:0405, and VBoxSVGA under
# 80ee:beef): VMware, VirtualBox, QEMU. Port I/O for the registers, MMIO
# for the framebuffer and the FIFO, no DMA. Serves the compositor the display
# wire (nonos_gfx_serve), as bga does; the kernel holds the endpoint to it alone.

CAPSULE_SLUG             := driver-vmware-svga
CAPSULE_HANDLE           := driver.vmware_svga0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_vmware_svga
CAPSULE_BIN_NAME         := driver_vmware_svga
CAPSULE_FEATURE          := nonos-capsule-driver-vmware-svga
CAPSULE_NAMESPACE        := systems.nonos.driver.vmware_svga0
CAPSULE_SERVICE_ENDPOINT := service:4280:driver.vmware_svga0
CAPSULE_REPLY_ENDPOINT   := reply:4281:endpoint.4294967351
# IPC|Memory|GraphicsSurfaceCreate|Driver|DeviceEnum|Mmio|Pio = 0x139018.
# Pio for the index and value ports. No Dma: on this path the adapter never
# reads guest memory, the driver copies into VRAM. No Irq.
CAPSULE_REQUIRED_CAPS    := 0x139018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100

include nonos-mk/capsule.mk
