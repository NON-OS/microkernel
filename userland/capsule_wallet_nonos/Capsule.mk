CAPSULE_SLUG             := wallet-nonos
CAPSULE_HANDLE           := app.nonos_wallet
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_wallet_nonos
CAPSULE_BIN_NAME         := wallet_nonos
CAPSULE_FEATURE          := nonos-capsule-wallet-nonos
CAPSULE_NAMESPACE        := systems.nonos.app.nonos_wallet
CAPSULE_SERVICE_ENDPOINT := service:4734:app.nonos_wallet
CAPSULE_REPLY_ENDPOINT   := reply:4735:endpoint.app.nonos_wallet.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4854:app.nonos_wallet.1 reply:4855:endpoint.app.nonos_wallet.1.reply service:4856:app.nonos_wallet.2 reply:4857:endpoint.app.nonos_wallet.2.reply
# CoreExec|Network|IPC|Memory|Crypto|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate
# FileSystem: the vault is saved and read through vfs (src/wallet/vault/vfs.rs),
# which serves only a holder of it.
CAPSULE_REQUIRED_CAPS    := 0x187d
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(), for its [APP] log lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_wallet_nonos

include nonos-mk/capsule.mk
