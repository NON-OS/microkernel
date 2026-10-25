CAPSULE_SLUG             := login
CAPSULE_HANDLE           := login
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_login
CAPSULE_BIN_NAME         := login
CAPSULE_FEATURE          := nonos-capsule-login
CAPSULE_NAMESPACE        := systems.nonos.login
CAPSULE_SERVICE_ENDPOINT := service:4416:login
CAPSULE_REPLY_ENDPOINT   := reply:4417:endpoint.login.reply
# IPC | Memory | GraphicsSurfaceCreate | Keyring
#   = 0x08 | 0x10 | 0x1000 | 0x10000000 = 0x10001018
# Keyring: the lock screen asks the keyring (src/setup/discover), and the
# kernel admits a sender to it only with the bit (registry/policy.rs).
# The lock screen registers and shares its own surface during setup
# (src/setup/run.rs); without that bit setup is refused and retried forever.
# Its size comes from the compositor's DISPLAY_INFO over IPC
# (src/setup/display.rs), not the kernel's display query, so it holds no
# GraphicsDisplayQuery. No CoreExec: login never asks its own pid or
# arguments.
CAPSULE_REQUIRED_CAPS    := 0x10001018
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_login

include nonos-mk/capsule.mk
