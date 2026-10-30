# input_router capsule. Drains the kernel input ring, normalizes
# events, and dispatches to the focused subscriber. No driver
# claims; only IPC + the MkInputEvent* surface.

CAPSULE_SLUG             := input-router
CAPSULE_HANDLE           := input_router
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_input_router
CAPSULE_BIN_NAME         := input_router
CAPSULE_FEATURE          := nonos-capsule-input-router
CAPSULE_NAMESPACE        := systems.nonos.input_router
CAPSULE_SERVICE_ENDPOINT := service:4320:input_router
CAPSULE_REPLY_ENDPOINT   := reply:4321:endpoint.input_router.reply
# IPC | Memory | InputConsumer = 0x08 | 0x10 | 0x10_0000_0000 = 0x1000000018
# InputConsumer alone drains and waits on the raw-input ring; the
# router never posts, so it holds no InputSource.
CAPSULE_REQUIRED_CAPS    := 0x1000000018
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_input_router

include nonos-mk/capsule.mk
