# model-fetch: `qwen get` and `qwen tiers`. Downloads the Qwen tiers a person
# picks from the NONOS model repository and feeds each file to the kernel,
# which seals it into the data volume and keeps it only if its SHA-256 is the
# signed pin. Spawned on demand as tool.model-fetch, parented to the Terminal
# that asked, like the command-line installer.
#
#   0x001 CoreExec      run at all
#   0x004 Network       reach mirrors through net.sockets, net.socks5 or net.anon
#   0x008 IPC           call those services and the crypto pool, write its output
#   0x010 Memory        the heap TLS records and a feed batch are held in
#   0x020 Crypto        the TLS handshake and record keys, through the crypto pool
#   0x400000000 StreamImport  feed a pinned file into the data volume
#
# No FileSystem: nothing on the volume is readable to it, so a capsule that
# holds sockets cannot read what the volume holds. No Debug: nothing reaches
# the serial log.
CAPSULE_SLUG             := model-fetch
CAPSULE_HANDLE           := tool.model-fetch
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_model_fetch
CAPSULE_BIN_NAME         := model-fetch
CAPSULE_FEATURE          := nonos-capsule-model-fetch
CAPSULE_NAMESPACE        := systems.nonos.tool.model-fetch
CAPSULE_SERVICE_ENDPOINT := service:4960:tool.model-fetch
CAPSULE_REPLY_ENDPOINT   := reply:4961:endpoint.tool.model-fetch.reply
CAPSULE_REQUIRED_CAPS    := 0x40000003D
CAPSULE_KERNEL_MIRROR    := src/userspace/tool_capsules/model_fetch
# The signed catalogue it embeds, written by mk/22-models.mk.
CAPSULE_EXTRA_DEPS       := $(TARGET_DIR)/models/catalogue.bin

include nonos-mk/capsule.mk
