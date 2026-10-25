# vmxnet3 — VMware's paravirtual NIC (15ad:07b0), in Workstation, Fusion and
# ESXi, and QEMU's `vmxnet3`. PCI MMIO (BAR0 doorbells, BAR1 registers) +
# DMA, polled, revision 1: one coherent control grant for the shared area,
# the queue table and every ring, and a receive and a transmit pool. Raw
# Ethernet frames only; network policy belongs to the stack capsules
# (net.core, net.l2), the only ones the kernel lets send to it.

CAPSULE_SLUG             := driver-vmxnet3
CAPSULE_HANDLE           := driver.vmxnet3_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_vmxnet3
CAPSULE_BIN_NAME         := driver_vmxnet3
CAPSULE_FEATURE          := nonos-capsule-driver-vmxnet3
CAPSULE_NAMESPACE        := systems.nonos.driver.vmxnet3_0
CAPSULE_SERVICE_ENDPOINT := service:4276:driver.vmxnet3_0
CAPSULE_REPLY_ENDPOINT   := reply:4277:endpoint.4294967349
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8038. No Irq: the driver
# polls and intrCtrl disables every interrupt. No Pio. Crypto (0x20) gates
# CryptoRandom, which the station address is drawn from; the draw fails
# closed, so without it the card never comes up.
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(), so `log vmxnet3`
# shows where a bring-up stopped.
CAPSULE_OPTIONAL_CAPS    := 0x100

include nonos-mk/capsule.mk
