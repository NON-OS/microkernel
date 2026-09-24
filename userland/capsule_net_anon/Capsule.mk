# net_anon: onion routing privacy capsule above net.tcp. It owns relay
# link sessions, circuit crypto and directory state for the Anyone
# network, and sits beside net_nym as the second onion transport.

CAPSULE_SLUG             := net-anon
CAPSULE_HANDLE           := net.anon
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_anon
CAPSULE_BIN_NAME         := net_anon
CAPSULE_FEATURE          := nonos-capsule-net-anon
CAPSULE_NAMESPACE        := systems.nonos.net.anon
CAPSULE_SERVICE_ENDPOINT := service:4472:net.anon
CAPSULE_REPLY_ENDPOINT   := reply:4473:endpoint.net.anon.reply
# 0x13d, named bit by bit, and every one of them is reached in the source:
#
#   0x001 CoreExec  run at all
#   0x004 Network   open sockets through net.tcp to relays and authorities
#   0x008 IPC       serve net.anon, and call net.tcp and the crypto pool
#   0x010 Memory    the heap a 1.8 MB consensus is parsed in
#   0x020 Crypto    SHA-256, HMAC, HKDF, X25519, Ed25519, RSA verify
#   0x100 Debug     the [ANON] bring-up markers on the serial log
#
# The same set as net-nym, which needs exactly these for the same reasons.
# Nothing here asks for FileSystem, Hardware or any graphics bit: this
# capsule never touches a device, a file or a surface.
CAPSULE_REQUIRED_CAPS    := 0x0013d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_anon

include nonos-mk/capsule.mk
