# std_proof — proof that unmodified std Rust runs on NONOS through the
# std platform layer: crates.io code, threads over MTSP, a process-local
# env, file I/O with seek over the vfs, and a TCP socket through the
# userland net stack, one PASS/FAIL serial line each. Built with
# -Zbuild-std=std and linked against the nonos-rt _start shim.

CAPSULE_BUILD_STD          := std,panic_abort

CAPSULE_SLUG               := std-proof
CAPSULE_HANDLE             := std_proof
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_std_proof
CAPSULE_BIN_NAME           := std_proof
CAPSULE_FEATURE            := nonos-capsule-std-proof
CAPSULE_NAMESPACE          := systems.nonos.std_proof
CAPSULE_SERVICE_ENDPOINT   := service:4502:std_proof
CAPSULE_REPLY_ENDPOINT     := reply:4503:endpoint.std_proof.reply
# CoreExec | IPC | Memory | FileSystem = 0x01 | 0x08 | 0x10 | 0x40 = 0x59,
# FileSystem for the file I/O proof, which std::fs sends to vfs. Debug (0x100)
# optional. The proof's println reaches the proc.<pid> inbox the terminal
# drains with or without Debug; Debug adds the copy on the serial line a
# smoke lane reads. The baked boot spawn folds it in through
# serial_debug_cap(), and a store install that asks for it is granted it.
CAPSULE_REQUIRED_CAPS      := 0x59
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS      := 0x100
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_std_proof

include nonos-mk/capsule.mk
