# app_store: the marketplace window.
#
# A GUI capsule that talks to one service. IPC reaches market.index,
# Memory backs the heap, and the two graphics capabilities register and
# present its surface. It asks for nothing else: it installs nothing
# itself, so it needs neither ForeignExec nor any store authority, and a
# window that can only read the catalogue cannot be turned into one that
# rewrites it.
#
# It may also ask for an install, which is not the right to perform
# one: AppInstall names a package and the personality does the work
# under its own manifest. The window never gains ForeignExec.
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate|AppInstall
CAPSULE_SLUG             := app_store
CAPSULE_HANDLE           := app.store
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_app_store
CAPSULE_BIN_NAME         := app_store
CAPSULE_FEATURE          := nonos-capsule-app-store
CAPSULE_NAMESPACE        := systems.nonos.app.store
CAPSULE_SERVICE_ENDPOINT := service:4940:app.store
CAPSULE_REPLY_ENDPOINT   := reply:4941:endpoint.app.store.reply
CAPSULE_REQUIRED_CAPS    := 0x40001819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_app_store

include nonos-mk/capsule.mk
