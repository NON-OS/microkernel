# socks5 — RFC 1928 front end that carries streams over the mixnet through
# `net.nym`. Network + IPC + Memory, Debug optional. No Crypto: net.nym
# does the cryptography, and this capsule makes no crypto syscall.

CAPSULE_SLUG             := socks5
CAPSULE_HANDLE           := net.socks5
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_socks5
CAPSULE_BIN_NAME         := socks5
CAPSULE_FEATURE          := nonos-capsule-socks5
CAPSULE_NAMESPACE        := systems.nonos.net.socks5
CAPSULE_SERVICE_ENDPOINT := service:4908:net.socks5
CAPSULE_REPLY_ENDPOINT   := reply:4909:endpoint.net.socks5.reply
CAPSULE_REQUIRED_CAPS    := 0x0001c
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_socks5

include nonos-mk/capsule.mk
