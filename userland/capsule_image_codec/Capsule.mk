CAPSULE_SLUG             := image-codec
CAPSULE_HANDLE           := image_codec
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_image_codec
CAPSULE_BIN_NAME         := image_codec
CAPSULE_FEATURE          := nonos-capsule-image-codec
CAPSULE_NAMESPACE        := systems.nonos.image_codec
CAPSULE_SERVICE_ENDPOINT := service:4412:image_codec
CAPSULE_REPLY_ENDPOINT   := reply:4413:endpoint.image_codec.reply
# IPC|Memory|GraphicsSurfaceCreate|GraphicsSurfaceMap = 0x3018. No
# GraphicsDisplayQuery: it decodes into surfaces it is handed and never asks
# the display anything.
CAPSULE_REQUIRED_CAPS    := 0x3018
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_image_codec

include nonos-mk/capsule.mk
