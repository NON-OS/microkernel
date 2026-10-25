# The development capsule set, booted at b/dev-capsule-set dbdc1b6cb

dbdc1b6cb is 13f8d1cec merged with main 87d670ef9. `tools/nonos-dev-image
--profile qemu` built and sealed qemu-dev with throwaway keys, in its own
copy of the tree; no key under `.keys/` was read. `head.txt` has the image's
sha256, `build-receipt.json` the build's own.

## The seal took the development set

`seal-capsules-step.txt`: step [2/6] ran `nix build .#capsules-dev`, and the
enrolment, the store pack and the declared-caps checks all read
`/nix/store/…-nonos-capsules-dev/…`. No Capsule.mk sets
CAPSULE_DEV_CARGO_FEATURES on this branch, so the set holds the same
capsules as `.#capsules`; what this shows is the development image taking
the development set end to end.

## Release images do not

`drvpaths.tsv`: every package's drvPath on main 87d670ef9 and on dbdc1b6cb,
with the real flake inputs. 218 of the 220 shared packages are identical,
every release image, the kernel, the bootloader and `.#capsules` among them;
no release image's derivation names a -dev capsule. The two that differ,
host-marketplace-index and host-tools, are host tools whose source is
nonos-mk/, which this branch edits. Only the branch has `capsules-dev`.

## The boot

`qemu.txt`: Intel IOMMU lane, 4 GiB, a fresh data disk, no TPM. Setup was
walked by keystroke over QMP with every default.

- `spawn-lines.txt`: the meter (payment, caps 0x10000079), nox_settle (caps
  0x3C) and the Terminal attested and spawned. The Terminal holds caps
  0xC100197D, its ceiling since 87d670ef9, which includes SpawnWindow.
- Neither the meter nor settle logged an exit, fault or reap in the six
  minutes after the desktop came up (`serial.log`, 1707 lines, to the end).
- No capsule failed to attest or spawn; the ones not spawned have no device
  in this machine. The line `[SPAWN] name=app.about[SOCKS5] open refused` is
  two capsules' serial output interleaved, not a failure.
- `20-desktop.png`: the desktop, menu bar and the full dock, with the live
  session dialog (3355 colours). `21-desktop-6min.png`: six minutes on.
