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
# LocalSign is what lets this capsule vouch for a package it installed.
# Without it MkLocalSign is refused, every package goes into the store
# with no trailer, and the exec gate then refuses all of them: the
# marketplace installs software that can never run. It is a narrow
# right. The trailer is made against the machine's own root, and that
# root verifies nothing until a human confirms a code on the console,
# which lapses at the next boot.
#
# = CoreExec 0x1 | IPC 0x8 | Memory 0x10 | Crypto 0x20 | Debug 0x100
#   | GfxQuery 0x800 | GfxCreate 0x1000
#   | ForeignExec 0x100000000 | LocalSign 0x200000000 = 0x300001939
# GfxQuery and GfxCreate are what any windowed app holds, for a guest's
# Wayland surface; it presents through the compositor, so no GfxPresent.

CAPSULE_SLUG             := linux
CAPSULE_HANDLE           := app.linux
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_linux
CAPSULE_BIN_NAME         := linux
CAPSULE_FEATURE          := nonos-capsule-linux
CAPSULE_NAMESPACE        := systems.nonos.app.linux
CAPSULE_SERVICE_ENDPOINT := service:4936:app.linux
CAPSULE_REPLY_ENDPOINT   := reply:4937:endpoint.app.linux.reply
# The install and run roles (src/userspace/capsule_linux/roles.rs) answer on
# their own endpoints. The spawn gate refuses any endpoint the signed manifest
# does not list, so without these every store install and every run of an
# installed package was refused before the capsule started. The two
# app.linux.term.N pairs are the slots a terminal runs a Qwen tier in, as its
# own child; 4944/4945 belong to a Linux guest (Guests.mk), so they start
# at 4946.
CAPSULE_INSTANCE_ENDPOINTS := service:4938:app.linux.install reply:4939:endpoint.app.linux.install.reply service:4942:app.linux.run reply:4943:endpoint.app.linux.run.reply service:4946:app.linux.term.1 reply:4947:endpoint.app.linux.term.1.reply service:4948:app.linux.term.2 reply:4949:endpoint.app.linux.term.2.reply
CAPSULE_REQUIRED_CAPS    := 0x304001979
# Network (bit 2) is optional: only the install role asks for it, to reach a
# package mirror through net.sockets (roles.rs). A guest runs without it.
CAPSULE_OPTIONAL_CAPS    := 0x4
CAPSULE_CAPS_CEILING     := 0x30400197D
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_linux

include nonos-mk/capsule.mk
