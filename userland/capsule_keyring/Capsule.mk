# keyring — userland service capsule. Callers carry
# `Capability::Keyring` and reach the capsule through IPC; the kernel
# admits a sender to the "keyring" endpoint only with it, and registers
# the endpoint only to an owner that holds what it asks of senders
# (src/services/registry/policy.rs, auth/owner_has_required.rs), so the
# keyring holds the bit too. The capsule needs IPC for `mk_ipc_*`, Memory
# for the heap, and Crypto for the kernel crypto syscall path it drives
# internally.

CAPSULE_SLUG             := keyring
CAPSULE_HANDLE           := keyring
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_keyring
CAPSULE_BIN_NAME         := keyring
CAPSULE_FEATURE          := nonos-capsule-keyring
CAPSULE_NAMESPACE        := systems.nonos.keyring
CAPSULE_SERVICE_ENDPOINT := service:4098:keyring
CAPSULE_REPLY_ENDPOINT   := reply:4099:endpoint.4294967298
# IPC | Memory | Crypto = 0x08 | 0x10 | 0x20 = 0x38
CAPSULE_REQUIRED_CAPS    := 0x10000038
CAPSULE_KERNEL_MIRROR    := src/security/keyring_capsule

include nonos-mk/capsule.mk
