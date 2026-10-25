# intel_display: Intel display engines from Haswell on, taken over at
# firmware's mode. PCI MMIO, polled; it scans out of granted memory through
# the GGTT, so it holds Dma, inside its own IOMMU domain. It serves the
# compositor the display wire bga and virtio-gpu speak.

CAPSULE_SLUG             := driver-intel-display
CAPSULE_HANDLE           := driver.intel_display0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_intel_display
CAPSULE_BIN_NAME         := driver_intel_display
CAPSULE_FEATURE          := nonos-capsule-driver-intel-display
CAPSULE_NAMESPACE        := systems.nonos.driver.intel_display0
CAPSULE_SERVICE_ENDPOINT := service:4574:driver.intel_display0
CAPSULE_REPLY_ENDPOINT   := reply:4575:endpoint.4294967331
# IPC (0x08): MkIpcRecvFrom and MkIpcReply, the wire; MkServiceLookup of
# `compositor`, the one pid handed its surfaces.
# Memory (0x10): MkMmap and MkMunmap of surface memory.
# GraphicsSurfaceCreate (0x1000): MkSurfaceRegister and MkSurfaceShare.
# DeviceEnum (0x8000): MkDeviceList, to find the graphics function.
# Driver (0x10000): MkDeviceClaim, MkDeviceRelease, and MkPciConfigRead and
# Write (Command; GGC at 0x50 read only).
# Mmio (0x20000): MkMmioMap of BAR0 and of the aperture window in BAR2.
# Dma (0x80000): MkDmaMap of the two scanout surfaces per output.
# = 0xB9018. No Pio, Irq, Network or InputSource: it polls.
CAPSULE_REQUIRED_CAPS    := 0xB9018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# shared bring-up's lines (absent, gave up) go through mk_debug.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/intel_display_capsule

include nonos-mk/capsule.mk
