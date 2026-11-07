CAPSULE_SLUG             := user-files
CAPSULE_HANDLE           := user_files
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_user_files
CAPSULE_BIN_NAME         := user_files
CAPSULE_FEATURE          := nonos-capsule-user-files
CAPSULE_NAMESPACE        := systems.nonos.user_files
CAPSULE_SERVICE_ENDPOINT := service:4424:user_files
CAPSULE_REPLY_ENDPOINT   := reply:4425:endpoint.user_files.reply
# IPC | Memory | UserFiles = 0x08 | 0x10 | 0x100_0000_0000
# No Debug: the broker logs nothing about the person's files, and the NO
# LOGS posture refuses the serial surface.
CAPSULE_REQUIRED_CAPS    := 0x10000000018
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_user_files

include nonos-mk/capsule.mk
