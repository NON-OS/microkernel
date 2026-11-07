# nsh.attest: per-handshake attestation and the host keys for nsh/1. Holds
# AttestRead, Crypto, FileSystem, IPC, Memory and CoreExec; NOT Network. The
# kernel refuses a TPM quote to any process that reaches the network, so the
# capsule that asks for quotes must not; for the same reason it is the one
# that holds the host's keys, made on the machine at first boot and sealed
# to its TPM, and reads the machine's records from the store. A release
# capsule, signed and enrolled with the rest; only the server profile's
# kernel embeds it (docs/nsh).

CAPSULE_SLUG             := nsh-attest
CAPSULE_HANDLE           := nsh.attest
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_nsh_attest
CAPSULE_BIN_NAME         := nsh_attest
CAPSULE_FEATURE          := nonos-capsule-nsh-attest
CAPSULE_NAMESPACE        := systems.nonos.nsh.attest
CAPSULE_SERVICE_ENDPOINT := service:4486:nsh.attest
CAPSULE_REPLY_ENDPOINT   := reply:4487:endpoint.nsh.attest.reply
# CoreExec | IPC | Memory | Crypto | FileSystem | AttestRead
# = 0x01 | 0x08 | 0x10 | 0x20 | 0x40 | 0x80000000. Crypto for the TPM's
# machine key and the generator, FileSystem for the sealed keys and the
# records. No Network, by the kernel's MkAttestDoc rule.
CAPSULE_REQUIRED_CAPS    := 0x80000079
# Debug, granted only by a build that compiles `capsule-serial-debug` (a
# development twin): the kernel mirror folds it in through serial_debug_cap(),
# so the first boot's key creation and every key fault are said on the
# serial console.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_nsh_attest

include nonos-mk/capsule.mk
