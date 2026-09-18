# install capsule. The desktop installer: writes the running image to a disk
# the user names and reads it back. Needs the graphics pair to open a window,
# DeviceEnum to list disks and to read the boot image through the kernel,
# Crypto for the GUIDs it mints, AttestRead to show what this boot verified,
# Admin for the reboot at the end.

CAPSULE_SLUG             := install
CAPSULE_HANDLE           := app.install
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_install
CAPSULE_BIN_NAME         := install
CAPSULE_FEATURE          := nonos-capsule-install
CAPSULE_NAMESPACE        := systems.nonos.app.install
CAPSULE_SERVICE_ENDPOINT := service:4932:app.install
CAPSULE_REPLY_ENDPOINT   := reply:4933:endpoint.app.install.reply
# CoreExec|IPC|Memory|Crypto|Admin|GraphicsDisplayQuery|GraphicsSurfaceCreate|DeviceEnum|AttestRead
CAPSULE_REQUIRED_CAPS    := 0x80009B39
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_install

include nonos-mk/capsule.mk
