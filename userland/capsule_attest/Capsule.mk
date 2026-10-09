CAPSULE_SLUG             := attest
CAPSULE_HANDLE           := attest
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_attest
CAPSULE_BIN_NAME         := attest
CAPSULE_FEATURE          := nonos-capsule-attest
CAPSULE_NAMESPACE        := systems.nonos.attest
CAPSULE_SERVICE_ENDPOINT := service:4444:attest
CAPSULE_REPLY_ENDPOINT   := reply:4445:endpoint.attest.reply
# CoreExec | IPC | Memory | AttestRead = 0x01 | 0x08 | 0x10 | 0x80000000
# = 0x80000019, what the kernel asks for when it spawns it. CoreExec (bit 0)
# is MkExit's; without it in the manifest the spawn gate refused the grant on
# every boot. AttestRead because it shows every live capsule's capability
# mask, which MkProcStat now hands only to a holder of it.
# Debug (0x100) deliberately absent: capsule_attest would lose all credibility
# if it emitted MkDebug markers. The NO LOGS posture is the point.
CAPSULE_REQUIRED_CAPS    := 0x80000019
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_attest

include nonos-mk/capsule.mk
