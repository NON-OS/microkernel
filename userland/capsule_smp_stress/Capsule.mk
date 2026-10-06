# smp_stress: holds every core busy with the wakes the scheduler must never
# lose (futex hand-offs, IPC round trips between threads, timed sleeps) and
# prints a line a minute and a verdict. Run from the Terminal as
# `smp_stress <seconds>`; one hour when no length is given.

CAPSULE_SLUG               := smp-stress
CAPSULE_HANDLE             := smp_stress
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_smp_stress
CAPSULE_BIN_NAME           := smp_stress
CAPSULE_FEATURE            := nonos-capsule-smp-stress
CAPSULE_NAMESPACE          := systems.nonos.smp_stress
CAPSULE_SERVICE_ENDPOINT   := service:5190:smp_stress
CAPSULE_REPLY_ENDPOINT     := reply:5191:endpoint.smp_stress.reply
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19: threads, IPC between
# them, and their stacks. No hardware, storage, network or filesystem.
CAPSULE_REQUIRED_CAPS      := 0x19
# Debug (0x100), for the copy of each line on the serial log.
CAPSULE_OPTIONAL_CAPS      := 0x100

include nonos-mk/capsule.mk
