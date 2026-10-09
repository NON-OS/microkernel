# input_proof capsule. Test-only proof surface for the input-stack
# end-to-end harness (Deliverable 2). Built on nonos_app_skeleton;
# subscribes to key/pointer/button events and emits [INPUT-PROOF]
# serial markers. Debug is optional, as in production apps: a kernel
# that compiles `capsule-serial-debug` grants it, so a profile that runs
# this proof must compile that feature for the CPL=3 surface to reach
# mk_debug.

CAPSULE_SLUG             := input-proof
CAPSULE_HANDLE           := app.input_proof
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_input_proof
CAPSULE_BIN_NAME         := input_proof
CAPSULE_FEATURE          := nonos-capsule-input-proof
CAPSULE_NAMESPACE        := systems.nonos.app.input_proof
CAPSULE_SERVICE_ENDPOINT := service:4790:app.input_proof
CAPSULE_REPLY_ENDPOINT   := reply:4791:endpoint.app.input_proof.reply
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate
# = 0x01|0x08|0x10|0x800|0x1000 = 0x1819
CAPSULE_REQUIRED_CAPS    := 0x1819
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_input_proof

include nonos-mk/capsule.mk
