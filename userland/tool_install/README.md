# tool_install

`install-cli` is the NONOS installer as a command-line program: it lists the disks the block
drivers serve and writes a whole NONOS disk to the one named, then reads every sector back and
prints a receipt. The disk layout, the block client and the confirmation word come from the same
crates the desktop installer uses (`nonos_disk`, `nonos_blk_client`), so the two agree about a
disk. It is a std program (`CAPSULE_BUILD_STD := std,panic_abort`) and reads its arguments and
its confirmation like any command-line tool. The handbook page is
[`docs/handbook/apps/installer.md`](../../docs/handbook/apps/installer.md).

## Commands

```
install-cli                      list the disks and what each holds (also: list)
install-cli write <word>         erase the disk whose word that is and install
            --yes                skip the typed confirmation
            --reboot             restart when the read-back passes
```

The word is the one `list` prints beside each disk: the last four characters of the disk's
serial when the part reports one, the bus name otherwise. Exit codes (`src/cli/mod.rs`): 0 done
or not confirmed, 1 usage, 2 no such disk or no working driver (also `list` with no disk
served), 3 the image, the entropy, the plan or the write failed, 4 the read-back failed.

## What `write` does

`src/cli/write.rs`, in order:

1. Finds the disk by its word among those `nonos_blk_client::survey` returns. The survey leaves
   off the disk the machine booted from, and lists one it cannot tell from it as not offered.
   A disk with 4096-byte blocks is refused (`Disk::refusal`).
2. Loads the running image from the kernel with `mk_install_source`: the bootloader, the
   kernel, the boot trailer, the boot-root record and the kernel approval when there is one
   (`src/cli/source.rs`). Without the trailer or the boot-root record it stops, since a disk
   without them would not boot. The loader configuration is the fixed
   `timeout=0\ndefault=nonos\n`.
3. Gathers what this boot carries into the new disk's store through `std::fs`, which on NONOS
   is the vfs (`src/cli/carry.rs`).
4. Takes entropy for the disk identifiers from `crypto_random` and plans the whole disk with
   `nonos_disk::Plan::new`. A disk too small is refused here, before anything is typed.
5. Prints the disk and every region the plan erases and writes, then asks for the word typed
   back (`src/cli/confirm.rs`). `--yes` skips the prompt.
6. Writes the plan in 4 MiB steps and reads it all back, printing a line every ten percent
   (`src/cli/run.rs`). A read-back mismatch names the region it fell in.
7. Prints the receipt: bytes written and verified, the disk and partition GUIDs, where each
   partition lies, the store's file count and the FAT32 geometry (`src/cli/receipt.rs`).
   `--reboot` then calls `mk_admin_reboot`.

## Capabilities and endpoints

`CAPSULE_REQUIRED_CAPS := 0x04008279`:

- `0x1` CoreExec, `0x8` IPC, `0x10` Memory.
- `0x20` Crypto: `crypto_random` for the disk identifiers.
- `0x40` FileSystem: what it carries comes through vfs.
- `0x200` Admin: the reboot.
- `0x8000` DeviceEnum: `MkInstallSource`, the read of the running image, is gated on it.
- `0x4000000` StoreWrite: raw sectors answer only the kernel and a StoreWrite holder.

`CAPSULE_OPTIONAL_CAPS := 0x100`: Debug, installed only on a `capsule-serial-debug` build. It
holds no graphics bit: it draws nothing. The kernel grants it `CLI_CAPS`
(`src/userspace/capsule_install/spawn.rs`), the desktop installer's set less its two display
bits.

Service `service:4934:tool.install`, reply `reply:4935:endpoint.tool.install.reply`.

## Build and registration

`make nonos-mk-install-cli` builds the ELF; `make nonos-mk-install-cli-sign` makes the
certificate, manifest and trailer. The kernel embeds them under the feature
`nonos-capsule-install-cli` and registers the program by hand in
`src/userspace/tool_capsules/registry.rs` as `tool.install`, rather than through
`userland/apps.list`, since it is not a crates.io tool.

## Not done yet

- Nothing in the tree runs `tool.install` at this commit. The terminal's `install` builtin goes
  to the market installer (`userland/capsule_terminal/src/jobs/classify.rs`), and no other
  capsule names the service.
- No host test covers this crate. The layout and plan it writes are `nonos_disk`'s.
