# vmbus — the VMBus core for Hyper-V guests (and QEMU's vmbus-bridge): the
# VMBus root claimed, one DMA grant for the contact's pages, the SynIC
# reached through the kernel's MkHv* calls, polled. Contacts the host,
# takes its channel offers and keeps the bus; gives storage and network
# channels records for their drivers (MkHvChannel) and opens none itself.
# Serves nothing; the kernel holds the endpoint to no capsule.

CAPSULE_SLUG             := driver-vmbus
CAPSULE_HANDLE           := driver.vmbus0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_vmbus
CAPSULE_BIN_NAME         := driver_vmbus
CAPSULE_FEATURE          := nonos-capsule-driver-vmbus
CAPSULE_NAMESPACE        := systems.nonos.driver.vmbus0
CAPSULE_SERVICE_ENDPOINT := service:4302:driver.vmbus0
CAPSULE_REPLY_ENDPOINT   := reply:4303:endpoint.4294967366
# IPC|Memory|DeviceEnum|Driver|Dma = 0x98018. Driver for the root's claim
# and the MkHv* calls, Dma for the contact's pages; no Mmio, Pio or Irq.
CAPSULE_REQUIRED_CAPS    := 0x98018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/vmbus_capsule

include nonos-mk/capsule.mk
