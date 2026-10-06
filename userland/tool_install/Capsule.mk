# install-cli: the installer from the terminal. Same crates, the same
# authority less the window's display bits, and the same typed-word
# confirmation as the window; built on the std PAL so
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
# CoreExec|IPC|Memory|Crypto|FileSystem|Admin|DeviceEnum|StoreWrite, Debug
# optional; FileSystem because what it carries comes through vfs, as the
# window's does (src/cli/carry.rs). No
# GraphicsDisplayQuery or GraphicsSurfaceCreate: it draws nothing, and the
# kernel grants it its own set (capsule_install::CLI_CAPS), not the window's.
CAPSULE_REQUIRED_CAPS      := 0x04008279
# Debug, installed only when the spawn grant names it, which the kernel's
# serial_debug_cap() does on a `capsule-serial-debug` build.
CAPSULE_OPTIONAL_CAPS      := 0x100
include nonos-mk/capsule.mk
