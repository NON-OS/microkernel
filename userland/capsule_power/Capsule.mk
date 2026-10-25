CAPSULE_SLUG             := power
CAPSULE_HANDLE           := power
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_power
CAPSULE_BIN_NAME         := power
CAPSULE_FEATURE          := nonos-capsule-power
CAPSULE_NAMESPACE        := systems.nonos.power
CAPSULE_SERVICE_ENDPOINT := service:4448:power
CAPSULE_REPLY_ENDPOINT   := reply:4449:endpoint.power.reply
# The wiped power-off, held to the desktop shell (HELD) and served on the
# same inbox as power: the kernel registers it beside it at spawn
# (src/userspace/capsule_power/power_off.rs). The one owner of the name.
CAPSULE_INSTANCE_ENDPOINTS := service:4446:power.off
# IPC | Memory | Admin = 0x08 | 0x10 | 0x200 = 0x218
# Admin gates AdminReboot + AdminShutdown.
# Debug deliberately absent: power_off must never leak to serial.
CAPSULE_REQUIRED_CAPS    := 0x218
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_power

include nonos-mk/capsule.mk
