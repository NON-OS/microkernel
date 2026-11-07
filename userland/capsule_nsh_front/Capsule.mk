# nsh.front: the in-guest nsh/1 host. Holds CoreExec, Network, IPC and
# Memory; NOT AttestRead, Crypto or FileSystem. It serves nsh/1 over a
# net.sockets stream listener and relays each handshake's attestation
# challenge, each signature the host makes and the machine's records through
# nsh.attest. Holding Network, it can never be quoted to directly and holds
# no host secret; that is the split. A release capsule, signed and enrolled
# with the rest; only the server profile's kernel embeds it (docs/nsh).

CAPSULE_SLUG             := nsh-front
CAPSULE_HANDLE           := nsh.front
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_nsh_front
CAPSULE_BIN_NAME         := nsh_front
CAPSULE_FEATURE          := nonos-capsule-nsh-front
CAPSULE_NAMESPACE        := systems.nonos.nsh.front
CAPSULE_SERVICE_ENDPOINT := service:4488:nsh.front
CAPSULE_REPLY_ENDPOINT   := reply:4489:endpoint.nsh.front.reply
# CoreExec | Network | IPC | Memory = 0x01 | 0x04 | 0x08 | 0x10. No AttestRead.
CAPSULE_REQUIRED_CAPS    := 0x0000001D
# Debug, granted only by a build that compiles `capsule-serial-debug` (a
# development twin): the kernel mirror folds it in through serial_debug_cap(),
# so a relay to nsh.attest that fails is said on the serial console.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_nsh_front

include nonos-mk/capsule.mk
