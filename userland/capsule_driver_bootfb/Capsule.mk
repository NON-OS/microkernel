# bootfb: the framebuffer firmware left, served on the display wire at its
# one mode; last of the display drivers, for any GPU no other one takes. It
# claims the kernel's boot framebuffer record (firmware bus, device 0x0001)
# and maps it write-combined; the CPU alone reads and writes it.

CAPSULE_SLUG             := driver-bootfb
CAPSULE_HANDLE           := driver.bootfb0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_bootfb
CAPSULE_BIN_NAME         := driver_bootfb
CAPSULE_FEATURE          := nonos-capsule-driver-bootfb
CAPSULE_NAMESPACE        := systems.nonos.driver.bootfb0
CAPSULE_SERVICE_ENDPOINT := service:4572:driver.bootfb0
CAPSULE_REPLY_ENDPOINT   := reply:4573:endpoint.4294967330
# IPC (0x08): MkServiceRegister of its own name, MkIpcRecvFrom and
# MkIpcReply, the wire; MkServiceLookup of `compositor`.
# Memory (0x10): MkMmap and MkMunmap of the surface's memory.
# GraphicsSurfaceCreate (0x1000): MkSurfaceRegister and MkSurfaceShare.
# DeviceEnum (0x8000): MkDeviceList, to find the record.
# Driver (0x10000): MkDeviceClaim and MkDeviceRelease of it.
# Mmio (0x20000): MkMmioMap of its bar 0, and MkBootFramebuffer.
# = 0x39018. No Dma: nothing but the CPU touches the framebuffer.
CAPSULE_REQUIRED_CAPS    := 0x39018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# shared bring-up's lines (absent, gave up) go through mk_debug.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/bootfb_capsule

include nonos-mk/capsule.mk
# Builds and is proven on the host, but the kernel it needs is not on main:
# the boot framebuffer record (8b7bcb660), MkBootFramebuffer (d8485f3d9) and
# the hand-off to a GPU driver (5ca0d371c) are on the the maintainers' branch. Remove,
# and add the include, mirror, feature and spawn, when those land.
CAPSULE_NOT_IN_IMAGE     := 1
