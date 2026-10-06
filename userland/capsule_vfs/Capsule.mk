# vfs: userland service capsule. CAP_VFS (FileSystem) is the caller-facing
# gate, which the capsule checks itself with MkCapCheck, not the
# capsule's bit. The capsule needs IPC for `mk_ipc_*`, Memory for the
# heap, and StoreWrite for its reads and writes of the capsule store.

CAPSULE_SLUG             := vfs
CAPSULE_HANDLE           := vfs
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_vfs
CAPSULE_BIN_NAME         := vfs
CAPSULE_FEATURE          := nonos-capsule-vfs
CAPSULE_NAMESPACE        := systems.nonos.vfs
CAPSULE_SERVICE_ENDPOINT := service:4104:vfs_pool
CAPSULE_REPLY_ENDPOINT   := reply:4105:endpoint.4294967301
# IPC | Memory | StoreWrite
#   = 0x08 | 0x10 | 0x4000000 = 0x4000018, Debug optional.
# No Keyring: the kernel tests it only for its keyring client, which only
# blockfs_volume::format_volume and mount_volume call, and nothing calls
# those; the data volume opens with the machine key.
# Debug: one line after seeding says how the store came up; a silent failure
# there reads as a shell timeout two layers up.
CAPSULE_REQUIRED_CAPS    := 0x04000018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/fs/vfs_capsule

# No audio ships in the image: Music's library is the person's own, in
# /home/nonos/music. Only the audio smoketest seeds /audio with its test tone,
# through its command-line vfs_CARGO_FEATURES=seed-audio-store.

# vfs no longer embeds any signed artifact: packages are read from the block
# device at seed time, and mk/40-run.mk keys the packer on $(std-proof_ARTIFACTS)
# so freshness is enforced there instead. The std_proof entries that used to live
# here are gone with the include_bytes! they existed for; the std_proof trailer
# in particular formed the root -> vfs -> trailer -> root cycle that make was
# dropping with a warning on every build.

include nonos-mk/capsule.mk
