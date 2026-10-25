# bga: Bochs/QEMU standard VGA display adapter (1234:1111). PCI MMIO only,
# polled: the DISPI registers behind BAR2 and the linear framebuffer in BAR0.
# It keeps the firmware's mode when it can draw it, and serves the
# compositor the display wire virtio-gpu speaks, from a surface in ordinary
# memory it copies each flushed rectangle out of, so the framebuffer
# mapping stays with the driver alone.

CAPSULE_SLUG             := driver-bga
CAPSULE_HANDLE           := driver.bga0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_bga
CAPSULE_BIN_NAME         := driver_bga
CAPSULE_FEATURE          := nonos-capsule-driver-bga
CAPSULE_NAMESPACE        := systems.nonos.driver.bga0
CAPSULE_SERVICE_ENDPOINT := service:4240:driver.bga0
CAPSULE_REPLY_ENDPOINT   := reply:4241:endpoint.4294967326
# IPC (0x08): MkServiceRegister, MkIpcRecvFrom and MkIpcReply, the wire;
# MkServiceLookup of `compositor`, the one pid handed the surface.
# Memory (0x10): MkMmap and MkMunmap of the surface's memory.
# GraphicsSurfaceCreate (0x1000): MkSurfaceRegister, MkSurfaceShare and
# MkSurfaceRelease of it, so the compositor can attach it.
# DeviceEnum (0x8000): MkDeviceList, to find the adapter.
# Driver (0x10000): MkDeviceClaim, MkDeviceRelease, and MkPciConfigRead and
# Write of the command register (Memory Space on, Bus Master off).
# Mmio (0x20000): MkMmioMap and MkMmioUnmap of BAR0 and BAR2.
# = 0x39018. No Dma: the adapter does no DMA. No Irq: the driver binds no
# line. No Pio: DISPI is reached through BAR2, not the 0x1CE ports.
CAPSULE_REQUIRED_CAPS    := 0x39018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# lines the shared bring-up logs, when no adapter is present (say_absent)
# and when it gives up, go through mk_debug. The kernel mirror folds it in
# through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/bga_capsule

include nonos-mk/capsule.mk
