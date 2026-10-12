# vmmouse — the VMware absolute pointer, VMware's and QEMU's (vmport),
# through the backdoor calls the kernel allows for it alone: no device,
# no grant, polled. Posts position, buttons and wheel onto the kernel
# input ring and serves nothing; the kernel holds the endpoint to no
# capsule.

CAPSULE_SLUG             := driver-vmmouse
CAPSULE_HANDLE           := driver.vmmouse0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_vmmouse
CAPSULE_BIN_NAME         := driver_vmmouse
CAPSULE_FEATURE          := nonos-capsule-driver-vmmouse
CAPSULE_NAMESPACE        := systems.nonos.driver.vmmouse0
CAPSULE_SERVICE_ENDPOINT := service:4300:driver.vmmouse0
CAPSULE_REPLY_ENDPOINT   := reply:4301:endpoint.4294967365
# IPC|Memory|Pio|InputSource = 0x300018. Pio for the backdoor; no Driver,
# DeviceEnum, Mmio, Dma or Irq: there is no device to claim.
CAPSULE_REQUIRED_CAPS    := 0x300018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/vmmouse_capsule

include nonos-mk/capsule.mk
