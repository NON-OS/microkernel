# install capsule. The desktop installer: writes a whole NONOS disk to a disk
# the user names (the running image, a store with what this boot carries, the
# disk plan) and reads it back. Needs the graphics pair to open a window,
# DeviceEnum to list disks and to read the boot image through the kernel,
# Crypto for the GUIDs it mints, Admin for the reboot at the end. The proofs
# it shows before a disk is chosen come from MkAttestStatus, MkAttestPolicy
# and MkBootAttest, which need no capability, and the count of capsules the
# kernel proved at spawn from MkAttestEntries, which needs AttestRead.
# What it carries it reads from the vfs over IPC, like any app, and vfs
# serves only a holder of FileSystem.

CAPSULE_SLUG             := install
CAPSULE_HANDLE           := app.install
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_install
CAPSULE_BIN_NAME         := install
CAPSULE_FEATURE          := nonos-capsule-install
CAPSULE_NAMESPACE        := systems.nonos.app.install
CAPSULE_SERVICE_ENDPOINT := service:4932:app.install
CAPSULE_REPLY_ENDPOINT   := reply:4933:endpoint.app.install.reply
# The brand crate the installer and setup share: its sources, faces and mark.
CAPSULE_EXTRA_DEPS       := $(shell find $(CAPSULE_DIR)/brand -type f -not -name "*.py" 2>/dev/null | sort)
# CoreExec|IPC|Memory|Crypto|FileSystem|Admin|GraphicsDisplayQuery|GraphicsSurfaceCreate|
# DeviceEnum|StoreWrite|AttestRead
CAPSULE_REQUIRED_CAPS    := 0x84009A79
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_install

include nonos-mk/capsule.mk
