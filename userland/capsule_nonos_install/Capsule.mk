# nonos_install capsule. Proof-carrying installer run from the live USB:
# composes the system on the target disk and hands back a root receipt.
# Console-only for now; the capset grows with each step that lands.

CAPSULE_SLUG             := nonos-install
CAPSULE_HANDLE           := app.nonos_install
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_nonos_install
CAPSULE_BIN_NAME         := nonos_install
CAPSULE_FEATURE          := nonos-capsule-nonos-install
CAPSULE_NAMESPACE        := systems.nonos.app.nonos_install
CAPSULE_SERVICE_ENDPOINT := service:4956:app.nonos_install
CAPSULE_REPLY_ENDPOINT   := reply:4957:endpoint.app.nonos_install.reply
# CoreExec | IPC | Memory | DeviceEnum = 0x8019: exactly what the landed
# steps call (src/asm/tags.S lists every syscall the program makes).
# The capset grows only when a step that needs more lands.
CAPSULE_REQUIRED_CAPS    := 0x00008019
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_nonos_install

include nonos-mk/capsule.mk
