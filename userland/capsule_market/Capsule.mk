# market — userland service capsule for the marketplace index.
# Standard userland-service bundle: IPC for `mk_ipc_*` and Memory for
# the heap. The release signature is checked in process by nonos_ed25519
# (src/verify/crypto.rs), not by a syscall, so it holds no Crypto.

CAPSULE_SLUG             := market
CAPSULE_HANDLE           := market
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_market
CAPSULE_BIN_NAME         := market
CAPSULE_FEATURE          := nonos-capsule-market
CAPSULE_NAMESPACE        := systems.nonos.market
CAPSULE_SERVICE_ENDPOINT := service:4106:market.index
CAPSULE_REPLY_ENDPOINT   := reply:4107:endpoint.4294967303
# CoreExec | IPC | Memory | FileSystem
#   = 0x01 | 0x08 | 0x10 | 0x40 = 0x59. CoreExec for MkGetPid
#   (src/boot_index.rs).
# FileSystem: the boot index is read through vfs (src/boot_index.rs), which
# serves only a holder of it. The kernel mirror does not name it; a required
# bit is installed whatever the spawn grant says.
CAPSULE_REQUIRED_CAPS    := 0x59
CAPSULE_KERNEL_MIRROR    := src/security/market_capsule

# The capsule embeds the signed catalogue, so a newer index has to
# rebuild it. Cargo tracks the include_bytes! path, but the make rule
# lists only sources, and without this line a freshly signed catalogue
# was silently left out of the image: the build succeeded, the boot
# succeeded, and the machine served the previous one.
CAPSULE_EXTRA_DEPS := $(TARGET_DIR)/market/index.bin

include nonos-mk/capsule.mk
