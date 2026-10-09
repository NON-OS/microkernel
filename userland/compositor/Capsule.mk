# compositor capsule. Owns scene state, damage tracking, the frame
# pacer, and the IPC client into driver.virtio_gpu0. No kernel
# coupling; everything lands through Mk* surface registry + IPC.

CAPSULE_SLUG             := compositor
CAPSULE_HANDLE           := compositor
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/compositor
CAPSULE_BIN_NAME         := compositor
CAPSULE_FEATURE          := nonos-capsule-compositor
CAPSULE_NAMESPACE        := systems.nonos.compositor
CAPSULE_SERVICE_ENDPOINT := service:4310:compositor
CAPSULE_REPLY_ENDPOINT   := reply:4311:endpoint.compositor.reply
CAPSULE_REQUIRED_CAPS    := 0x7818
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_compositor

include nonos-mk/capsule.mk
