# setup_wizard capsule. First-boot setup wizard: attaches a fullscreen
# compositor surface, grabs the keyboard, walks the user through setup
# (keyboard, name, time zone, mode, network, wallpaper, Qwen model), then
# exits so the kernel brings up the desktop. Same leaf-renderer capset as
# input_probe (no SurfaceMap/Present), plus EnrolDevRoot: setup is where a
# person lets this machine run what it installs, and no app window holds
# that right. Plus Crypto: the TPM-derived key that seals a Wi-Fi network
# setup remembers. Plus FileSystem: what setup keeps, the consent and the
# remembered network, goes through vfs, which serves only a holder of it.

CAPSULE_SLUG             := setup-wizard
CAPSULE_HANDLE           := app.setup_wizard
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_setup_wizard
CAPSULE_BIN_NAME         := setup_wizard
CAPSULE_FEATURE          := nonos-capsule-setup-wizard
CAPSULE_NAMESPACE        := systems.nonos.app.setup_wizard
CAPSULE_SERVICE_ENDPOINT := service:4794:app.setup_wizard
CAPSULE_REPLY_ENDPOINT   := reply:4795:endpoint.app.setup_wizard.reply
CAPSULE_REQUIRED_CAPS    := 0x8001879
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_setup_wizard
# build.rs reads the Qwen pins, so a changed pin rebuilds setup.
CAPSULE_EXTRA_DEPS       := $(CAPSULE_DIR)/build.rs \
                            $(shell find userland/capsule_install/brand -type f -not -name "*.py" 2>/dev/null | sort) \
                            $(wildcard userland/capsule_linux/src/linux/file/models/pinned*.rs)

include nonos-mk/capsule.mk
