# NONOS capsule_wallpaper_catalog
# Read-only asset vendor for the 63 Full-HD wallpaper JPEGs.

CAPSULE_SLUG             := wallpaper_catalog
CAPSULE_HANDLE           := wallpaper_catalog
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_wallpaper_catalog
CAPSULE_BIN_NAME         := wallpaper_catalog
CAPSULE_FEATURE          := nonos-capsule-wallpaper-catalog
CAPSULE_NAMESPACE        := systems.nonos.wallpaper_catalog
CAPSULE_SERVICE_ENDPOINT := service:4110:wallpaper_catalog
CAPSULE_REPLY_ENDPOINT   := reply:4111:endpoint.wallpaper_catalog.reply
# CoreExec|IPC|Memory|FileSystem = 0x1 | 0x8 | 0x10 | 0x40. FileSystem: the
# wallpapers are read out of the store's collection through vfs, the ones
# asked for only. CoreExec: MkGetPid, the pid vfs is told the read is for.
CAPSULE_REQUIRED_CAPS    := 0x59
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_wallpaper_catalog

include nonos-mk/capsule.mk
