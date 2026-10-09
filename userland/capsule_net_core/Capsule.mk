# net_core: smoltcp-backed network core. Drives the bound NIC driver
# capsule (a Wi-Fi card first, then a wired one, whichever has link) via a
# phy::Device and serves the net.* services from one smoltcp Interface.
# Identity service `net.core`; real service names registered at runtime.

CAPSULE_SLUG             := net-core
CAPSULE_HANDLE           := net.core
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_core
CAPSULE_BIN_NAME         := net_core
CAPSULE_FEATURE          := nonos-capsule-net-core
CAPSULE_NAMESPACE        := systems.nonos.net.core
CAPSULE_SERVICE_ENDPOINT := service:4480:net.core
CAPSULE_REPLY_ENDPOINT   := reply:4481:endpoint.net.core.reply
# CoreExec|IPC|Memory|Crypto|Network|FileSystem|RegisterService
# FileSystem: autojoin reads the saved Wi-Fi networks through vfs
# (nonos_wifi_client::load), which serves only a holder of it.
CAPSULE_REQUIRED_CAPS    := 0x0047d
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_core

include nonos-mk/capsule.mk
