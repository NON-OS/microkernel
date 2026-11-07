# NONOS capsule_policy: system policy store.
# RAM-ephemeral live settings, IPC surface for capsule readers /
# writers, MkAdminPolicyPush to push kernel-mirrored fields.

CAPSULE_SLUG             := policy
CAPSULE_HANDLE           := policy
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_policy
CAPSULE_BIN_NAME         := policy
CAPSULE_FEATURE          := nonos-capsule-policy
CAPSULE_NAMESPACE        := systems.nonos.policy
CAPSULE_SERVICE_ENDPOINT := service:4108:policy
CAPSULE_REPLY_ENDPOINT   := reply:4109:endpoint.policy.reply
# CoreExec | IPC | Memory | FileSystem | PolicyPush
#   = 0x01 | 0x08 | 0x10 | 0x40 | 0x40000000000 = 0x40000000059
# PolicyPush gates AdminPolicyPush, the capsule's one kernel call. No Admin.
# FileSystem: kept settings are restored through vfs (src/restore/tick.rs)
# and kept there when they change (src/keep), which vfs serves only a
# holder of it.
CAPSULE_REQUIRED_CAPS    := 0x40000000059
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(). It carries the
# "[POLICY]" lines (a restore refused or done, a vfs pass late, keeping
# refused by the kernel); without it each is a refused MkDebug and the line
# is lost. Policy holds no key, input or secret, so the kernel does not
# withhold it.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_policy

include nonos-mk/capsule.mk
