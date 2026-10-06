# payment — userland service capsule. Issues signed NOX install
# receipts. Holds no key material: it assembles receipt fields and
# calls capsule_keyring over IPC to sign. Needs IPC for mk_ipc_* and
# Memory for the heap; it never drives a kernel crypto syscall
# directly (signing happens inside the keyring).

CAPSULE_SLUG             := payment
CAPSULE_HANDLE           := payment
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_payment
CAPSULE_BIN_NAME         := payment
CAPSULE_FEATURE          := nonos-capsule-payment
CAPSULE_NAMESPACE        := systems.nonos.payment
# 4110 and 4111 belonged to wallpaper_catalog, which the kernel spawn plan
# hardcodes; whichever of the two registered second would have failed. The
# reply name stays the kernel endpoint this capsule sends replies to
# (KERNEL_REPLY_ENDPOINT, 0x1_0000_001B), as the driver capsules do;
# 0x1_0000_0010 was driver_hda's reply inbox.
CAPSULE_SERVICE_ENDPOINT := service:4114:payment
CAPSULE_REPLY_ENDPOINT   := reply:4115:endpoint.4294967323
# IPC | Memory = 0x08 | 0x10 = 0x18
CAPSULE_REQUIRED_CAPS    := 0x18

include nonos-mk/capsule.mk
