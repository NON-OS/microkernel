# acpi: the platform's AML, run in a capsule. Reaches firmware's registers
# only through the kernel's ACPI grant rule (MkAcpiTable, MkAcpiRegion,
# MkAcpiPort, MkAcpiPci); claims the SCI record and nothing else; no DMA.
# IPC|Memory|Admin|DeviceEnum|Driver|Irq
# = 0x08|0x10|0x200|0x8000|0x10000|0x40000 = 0x58218
# Admin stands in for the Firmware bit the design notes proposes.

CAPSULE_SLUG             := acpi
CAPSULE_HANDLE           := acpi
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_acpi
CAPSULE_BIN_NAME         := acpi
CAPSULE_FEATURE          := nonos-capsule-acpi
CAPSULE_NAMESPACE        := systems.nonos.acpi
CAPSULE_SERVICE_ENDPOINT := service:4530:acpi
CAPSULE_REPLY_ENDPOINT   := reply:4531:endpoint.acpi.reply
CAPSULE_REQUIRED_CAPS    := 0x58218
# Debug (0x100), for its serial lines in a build with capsule serial debug.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_acpi

include nonos-mk/capsule.mk
