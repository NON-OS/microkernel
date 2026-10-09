# shield capsule. nonos.shield, the NOX Shield wallet the phone apps run
# (userland/shield_core), as the service the wallet window calls: it opens
# the shield from the wallet's words, reads the pool, sends deposits, proves
# private payments and withdrawals on the machine and settles them. Built
# against the std layer: the prover proves on every core through its threads.
#
#   0x001  CoreExec    run at all
#   0x004  Network     every read and send, over Nym or Anyone only
#   0x008  IPC         the wallet window's calls, net.socks5 and net.anon
#   0x010  Memory      the prover's heap: about 0.94 GB for a proof from
#                      the shipped periodic cache, 1.2 GB held after the
#                      second (the std heap keeps what it maps), 1.6 GB
#                      for a proof with no cache
#   0x020  Crypto      CryptoRandom for blindings and the proof's entropy;
#                      MachineKey for the store's sealed file key
#   0x040  FileSystem  the sealed note store under /data/shield

CAPSULE_BUILD_STD          := std,panic_abort

CAPSULE_SLUG               := shield
CAPSULE_HANDLE             := nonos.shield
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_shield
CAPSULE_BIN_NAME           := shield
CAPSULE_FEATURE            := nonos-capsule-shield
CAPSULE_NAMESPACE          := systems.nonos.shield
CAPSULE_SERVICE_ENDPOINT   := service:5012:nonos.shield
CAPSULE_REPLY_ENDPOINT     := reply:5013:endpoint.nonos.shield.reply
# CoreExec|Network|IPC|Memory|Crypto|FileSystem
CAPSULE_REQUIRED_CAPS      := 0x7d
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(), for the [shield lines.
CAPSULE_OPTIONAL_CAPS      := 0x100
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_shield

include nonos-mk/capsule.mk
