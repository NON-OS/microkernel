# nox_settle — the chain's reader and broadcaster for the meter, the wallet
# and the store. It holds no key: it reads views and logs, quotes
# intents, fetches release bytes and verifies their hash, and sends what a
# key holder signed. Every connection goes over Route::for_wallet(), the Nym
# mixnet or the Anyone network, never Direct. IPC, Memory, Network, and
# Crypto for nonos_tls's randomness.

CAPSULE_SLUG             := nox-settle
CAPSULE_HANDLE           := nox_settle
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_nox_settle
CAPSULE_BIN_NAME         := nox_settle
CAPSULE_FEATURE          := nonos-capsule-nox-settle
CAPSULE_NAMESPACE        := systems.nonos.nox_settle
# 4962 to 4969 belong to the Linux guests; 5200 to 5207 are the market
# services'. The reply name is this capsule's own kernel reply
# endpoint, 0x1_0000_001C.
CAPSULE_SERVICE_ENDPOINT := service:5200:nox_settle
CAPSULE_REPLY_ENDPOINT   := reply:5201:endpoint.4294967332
# Network | IPC | Memory | Crypto = 0x04 | 0x08 | 0x10 | 0x20
CAPSULE_REQUIRED_CAPS    := 0x3C
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_nox_settle

include nonos-mk/capsule.mk
