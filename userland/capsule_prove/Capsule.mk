# prove capsule. nonos.prove, the anonymous device proof: proves on the booted
# OS that this machine runs an enrolled bootloader and kernel and is an
# enrolled device, without naming it. A window on nonos_app_skeleton, opened
# on demand only: it holds the prover's heap, 2816 MiB, while it is open, and
# closing it exits it, which zeroizes everything it held.
#
#   0x001         CoreExec               run at all
#   0x008         IPC                    the window: compositor, wm, input router
#   0x010         Memory                 the prover's heap
#   0x020         Crypto                 CryptoRandom for the blinding, CryptoHash
#                                        for the output file's digest
#   0x040         FileSystem             read the request and the registry
#                                        transcript from the data volume
#   0x800         GraphicsDisplayQuery   the window
#   0x1000        GraphicsSurfaceCreate  the window
#   0x400000000   StreamImport           leave the proof on the data volume,
#                                        kept only under its SHA-256
#   0x800000000   DeviceSecret           MkDeviceSecret, MkBootSlots, MkEnroll;
#                                        no other capsule holds it
#
# No Network: the person brings the request and the transcript in. The
# nonos.prove.fetch capsule meant to fetch them does not exist yet. No Debug: nothing it learns reaches the serial log.

CAPSULE_SLUG             := prove
CAPSULE_HANDLE           := app.prove
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_prove
CAPSULE_BIN_NAME         := prove
CAPSULE_FEATURE          := nonos-capsule-prove
CAPSULE_NAMESPACE        := systems.nonos.app.prove
CAPSULE_SERVICE_ENDPOINT := service:4950:app.prove
CAPSULE_REPLY_ENDPOINT   := reply:4951:endpoint.app.prove.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4952:app.prove.1 reply:4953:endpoint.app.prove.1.reply
# CoreExec|IPC|Memory|Crypto|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate|StreamImport|DeviceSecret
CAPSULE_REQUIRED_CAPS    := 0xC00001879
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(), for its [APP] log lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_prove

include nonos-mk/capsule.mk
