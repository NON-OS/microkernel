# Design principles: Amnesic by default

Part of [Design principles](design-principles.md).

## Amnesic by default

Every boot is [amnesic](glossary.md#amnesic-boot) unless the person chooses to
install, in first-boot setup or from the boot menu's install entry. With the
`install` key set to false, an image has no setup and no installer at all
(`nonos.toml:22-25`). First-boot setup is the only thing that records the choice
to keep anything (`installFeatures` in `tools/nix/config.nix:43-46`). When setup
ends, init hands the screen to the installer only if the person chose to
install, or if setup ended without an answer on a boot started from the menu's
install entry (`poll` in `src/userspace/init/supervisor/after_setup.rs:17-48`).

A file a program asks to keep goes to the [package store](glossary.md#store),
which is written to the disk as it is, with no encryption (`append` in
`userland/capsule_vfs/src/blk/store_write.rs:39-84`). Qwen models and other
imported files go to a [data volume](glossary.md#data-volume). On a live stick,
whose disk plan keeps no volume, that volume is held in RAM
(`open_session_volume` in `src/fs/blockfs_volume/open_machine.rs:46-53`). On an
installed disk every sector is sealed with ChaCha20-Poly1305 (`seal` and
`aead_encrypt` in `src/fs/cryptoblock/seal.rs:18-46`), under a [machine
key](glossary.md#machine-key) the [TPM](glossary.md#tpm) derives for this
machine in this boot state (`KEY_LABEL` in
`src/fs/blockfs_volume/open_machine.rs:17-38`). The kernel can instead seal a
random volume key with a passphrase stretched by Argon2id (`passphrase_volume`
in `src/fs/blockfs_volume/passphrase.rs:17-32`), but no capsule in this release
asks for one.

Shutting down and restarting, the `AdminShutdown` and `AdminReboot` syscalls,
both run the [ZeroState](glossary.md#zerostate) wipe first (`shutdown` in
`src/syscall/dispatch/router/admin/shutdown.rs:20-27`, `reboot` in
`src/syscall/dispatch/router/admin/reboot.rs:20-28`): the other CPUs are
stopped, then memory is wiped, then the firmware is called (`terminate` in
`src/security/zerostate/terminate.rs:21-39`). The wipe stops claimed devices and
wipes their DMA buffers before it wipes process memory, kernel stacks,
filesystem caches, the kernel's key vault, the RAM log and the heap
(`zerostate_shutdown_wipe` in
`src/security/hardening/memory_sanitization/api.rs:59-109`).

The limits: stopping the other CPUs is best effort. On an interrupt controller
that refuses the broadcast, the wipe covers what the running core can reach
(`terminate` in `src/security/zerostate/terminate.rs:31-35`). Kernel statics
outside the heap are not wiped, the data volume key in `VOLUME` among them
(`src/fs/blockfs_volume/state.rs:21-26`). A live stick's in-memory volume is not
wiped either: `Ram` keeps its sealed sectors in frames outside the heap
(`src/fs/cryptoblock/ram.rs:41-47`). In 0.9.2 the desktop cannot shut down or
restart, so the installer's restart is the one path that runs the wipe
(`mk_admin_reboot` in `userland/capsule_install/src/install/event/after.rs:38`).
A kernel panic halts every CPU without the wipe (`panic` in
`src/boot/panic/handler.rs:41-64`), and cutting the power skips it too.
