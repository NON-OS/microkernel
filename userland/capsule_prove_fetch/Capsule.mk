# prove-fetch capsule. nonos.prove.fetch (plan/14): carries kind-1 proof
# containers between app.store or market.index and the proof relay, over Nym
# or Anyone and never directly. It uploads only bytes that are a container,
# under the anchor it recomputes, and keeps a download only when it hashes
# to the anchor asked for, so it is never a general egress.
#
#   0x01   CoreExec   run at all
#   0x04   Network    the route to the relay
#   0x08   IPC        its endpoint, net.socks5 or net.anon
#   0x10   Memory     a container in flight, at most 1 MiB
#   0x20   Crypto     CryptoRandom for TLS
#
# No DeviceSecret and no FileSystem: it sees containers, never a witness.

CAPSULE_SLUG             := prove-fetch
CAPSULE_HANDLE           := nonos.prove.fetch
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_prove_fetch
CAPSULE_BIN_NAME         := prove-fetch
CAPSULE_FEATURE          := nonos-capsule-prove-fetch
CAPSULE_NAMESPACE        := systems.nonos.prove.fetch
CAPSULE_SERVICE_ENDPOINT := service:5202:nonos.prove.fetch
CAPSULE_REPLY_ENDPOINT   := reply:5203:endpoint.nonos.prove.fetch.reply
# CoreExec|Network|IPC|Memory|Crypto
CAPSULE_REQUIRED_CAPS    := 0x3D
# Debug, granted only by a build that compiles `capsule-serial-debug`.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_prove_fetch

include nonos-mk/capsule.mk
