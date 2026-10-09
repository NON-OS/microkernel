# attack capsule. Test only, for the attack suite,
# and not yet wired: no Cargo feature, kernel mirror or make include names it:
# a capsule that tries what an attacker would, and prints one [ATTACK] line
# per attempt with the kernel's answer. It holds the least a capsule runs on,
# so every attempt below is one it has no right to:
#
#   0x001  CoreExec  run at all
#   0x008  IPC       reach a service, which is where it tries to reach one
#                    that takes Network
#   0x010  Memory    MkMmap, which is where it asks for a fixed kernel address
#   0x100  Debug     MkDebug, its only way to say what happened
#
# No Network, no Mmio, no Hardware, no Admin, no RegisterService.

CAPSULE_SLUG               := attack
CAPSULE_HANDLE             := test.attack
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_attack
CAPSULE_BIN_NAME           := attack
CAPSULE_FEATURE            := nonos-capsule-attack
CAPSULE_NAMESPACE          := systems.nonos.test.attack
CAPSULE_SERVICE_ENDPOINT   := service:4954:test.attack
CAPSULE_REPLY_ENDPOINT     := reply:4955:endpoint.test.attack.reply
# CoreExec|IPC|Memory|Debug
CAPSULE_REQUIRED_CAPS      := 0x119
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_attack
CAPSULE_BUILD_STD          := core

include nonos-mk/capsule.mk
