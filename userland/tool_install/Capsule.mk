# install-cli: the installer from the terminal. Same crates, same authority
# and the same typed-word confirmation as the window; built on the std PAL so
# it reads its arguments and its confirmation like any command-line tool, and
# registered by hand in the kernel's tool registry rather than through
# apps.list, because it is ours and not a crates.io crate.

CAPSULE_BUILD_STD          := std,panic_abort
CAPSULE_SLUG               := install-cli
CAPSULE_HANDLE             := tool.install
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/tool_install
CAPSULE_BIN_NAME           := install-cli
CAPSULE_FEATURE            := nonos-capsule-install-cli
CAPSULE_NAMESPACE          := systems.nonos.tool.install
CAPSULE_SERVICE_ENDPOINT   := service:4934:tool.install
CAPSULE_REPLY_ENDPOINT     := reply:4935:endpoint.tool.install.reply
# CoreExec|IPC|Memory|Crypto|Admin|GraphicsDisplayQuery|GraphicsSurfaceCreate|DeviceEnum|AttestRead
CAPSULE_REQUIRED_CAPS      := 0x80009B39

include nonos-mk/capsule.mk
