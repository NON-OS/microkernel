CAPSULE_SLUG             := terminal
CAPSULE_HANDLE           := app.terminal
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_terminal
CAPSULE_BIN_NAME         := terminal
CAPSULE_FEATURE          := nonos-capsule-terminal
CAPSULE_NAMESPACE        := systems.nonos.app.terminal
CAPSULE_SERVICE_ENDPOINT := service:4722:app.terminal
CAPSULE_REPLY_ENDPOINT   := reply:4723:endpoint.app.terminal.reply
# Endpoints for up to three extra on-demand windows. Each pair is declared in
# the signed manifest so a runtime-spawned instance can register its own
# service and receive its own compositor window.
CAPSULE_INSTANCE_ENDPOINTS := \
	service:4740:app.terminal.1 reply:4741:endpoint.app.terminal.1.reply \
	service:4742:app.terminal.2 reply:4743:endpoint.app.terminal.2.reply \
	service:4744:app.terminal.3 reply:4745:endpoint.app.terminal.3.reply
# CoreExec|Network|IPC|Memory|Crypto|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate|
# AppInstall|AttestRead. AppInstall is the right to ask the system to install
# or uninstall a marketplace listing and to read where that stands, which
# `market install`, `market uninstall`, `market list` and `market info` do:
# the person at the prompt asks, as at the Marketplace window's button.
CAPSULE_REQUIRED_CAPS    := 0xc000187d
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(), for its [APP] log lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_terminal

include nonos-mk/capsule.mk
