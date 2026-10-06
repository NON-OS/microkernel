# audio.server — system audio mixing service. IPC-only: no hardware,
# no broker claim, no MMIO/IRQ/DMA. It receives client play/PCM
# requests, mixes them additively into an S16 buffer, and forwards
# the result to the driver.hda0 PCM sink over IPC.

CAPSULE_SLUG             := audio
CAPSULE_HANDLE           := audio.server
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_audio
CAPSULE_BIN_NAME         := audio_server
CAPSULE_FEATURE          := nonos-capsule-audio
CAPSULE_NAMESPACE        := systems.nonos.audio.server
CAPSULE_SERVICE_ENDPOINT := service:4872:audio.server
CAPSULE_REPLY_ENDPOINT   := reply:4873:endpoint.4294967321
# IPC | Memory = 0x18
CAPSULE_REQUIRED_CAPS    := 0x18
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/audio_capsule
include nonos-mk/capsule.mk
