# simplex.ui, the native SimpleX chat window. No Network bit: it reaches the
# simplex-chat core only through the channel the 2026-10-27 letter asks S1
# for (SERVE_ONE simplex.ui). Until then it replays a recorded session.
#
# Included from mk/20-build.mk and started from the desktop's SimpleX tile.
# A build with nonos-capsule-simplex-ui needs the publisher keys and the
# signed certificate, manifest and trailer under nonos-data/trust, which only
# the owner makes (README.md, Wiring).

CAPSULE_SLUG             := simplex-ui
CAPSULE_HANDLE           := app.simplex
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_simplex_ui
CAPSULE_BIN_NAME         := simplex_ui
CAPSULE_FEATURE          := nonos-capsule-simplex-ui
CAPSULE_NAMESPACE        := systems.nonos.app.simplex
CAPSULE_SERVICE_ENDPOINT := service:5014:app.simplex
CAPSULE_REPLY_ENDPOINT   := reply:5015:endpoint.app.simplex.reply
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1819
# Debug, granted only by a build that compiles `capsule-serial-debug`.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_simplex_ui

include nonos-mk/capsule.mk
