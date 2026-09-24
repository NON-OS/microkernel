# NØNOS userland: capsule_linux
# eK@nonos.systems
#
# The Linux personality. ForeignExec is the whole point of this capsule
# and no other capsule holds it: it is the right to create a process the
# kernel has not verified, build its address space, and answer the calls
# it makes. Crypto is there for getrandom, Debug for the guest's console
# until the file layer carries it, and the rest is the ordinary capsule
# floor.
#
# = CoreExec 0x1 | IPC 0x8 | Memory 0x10 | Crypto 0x20 | Debug 0x100
#   | ForeignExec 0x100000000 = 0x100000139

CAPSULE_SLUG             := linux
CAPSULE_HANDLE           := app.linux
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_linux
CAPSULE_BIN_NAME         := linux
CAPSULE_FEATURE          := nonos-capsule-linux
CAPSULE_NAMESPACE        := systems.nonos.app.linux
CAPSULE_SERVICE_ENDPOINT := service:4936:app.linux
CAPSULE_REPLY_ENDPOINT   := reply:4937:endpoint.app.linux.reply
CAPSULE_REQUIRED_CAPS    := 0x100000139
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_linux

include nonos-mk/capsule.mk
