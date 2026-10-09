# shield vectors. Test 5: the four pinned production wallet vectors
# (STARKs spec/wallet-vectors-not-before, copied under vectors/) proved on
# the machine by the nox_prover nonos.shield links, each proof and its
# format 7 form held to the pinned bytes, with the time and the memory each
# took on the serial line. Development images only: its kernel feature is
# refused beside a release loader (tools/nix/config.nix) and beside
# nonos-release (src/userspace/capsule_shield_vectors).
#
#   0x001  CoreExec    run at all
#   0x008  IPC         the prover's threads (MkThreadSpawn)
#   0x010  Memory      the prover's heap, up to about 1.5 GB
#   0x100  Debug       optional: the result lines on the serial console

CAPSULE_BUILD_STD          := std,panic_abort

CAPSULE_SLUG               := shield-vectors
CAPSULE_HANDLE             := shield_vectors
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_shield_vectors
CAPSULE_BIN_NAME           := shield_vectors
CAPSULE_FEATURE            := nonos-capsule-shield-vectors
CAPSULE_NAMESPACE          := systems.nonos.shield_vectors
CAPSULE_SERVICE_ENDPOINT   := service:4988:shield_vectors
CAPSULE_REPLY_ENDPOINT     := reply:4989:endpoint.shield_vectors.reply
# CoreExec|IPC|Memory
CAPSULE_REQUIRED_CAPS      := 0x19
CAPSULE_OPTIONAL_CAPS      := 0x100
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_shield_vectors
# A development test: never signed or enrolled by a make lane, enrolled by
# the seal for development images only.
CAPSULE_DEV_ONLY           := 1

include nonos-mk/capsule.mk
