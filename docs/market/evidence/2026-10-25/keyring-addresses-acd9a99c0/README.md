# The keyring reading its addresses from nonos_nox_config, booted at b/keyring-addresses acd9a99c0

acd9a99c0 is c56071bbc (D12) merged with main 0e98f0cda. `tools/nonos-dev-image
--profile qemu` built and sealed qemu-dev with throwaway keys in its own copy
of the tree; no key under `.keys/` was read. `head.txt` has the image's
sha256, `build-receipt.json` the build's own. `qemu.txt`: Intel IOMMU lane,
4 GiB, a fresh data disk, no TPM. Setup was walked by keystroke with every
default.

## What it shows

- `keyring-lines.txt`: `[ZK-ATTEST] ok keyring vendor`, then `[SPAWN]
  name=keyring pid=0x3 caps=0x10000038` and the run queue.
- No exit, fault or reap of the keyring, or of any capsule, in the six
  minutes the desktop was up (`serial.log`, 1752 lines, to the end of the run).
- No capsule failed to attest or spawn; those not spawned have no device here.
- `20-desktop.png`: the desktop, menu bar and the full dock, with the live
  session dialog (3348 colours). `21-desktop-6min.png`: six minutes on.

## Signing on the device

No typed signing request can reach the keyring on a device until the
marketplace contracts are deployed. The wallet's `sign_typed`
(`capsule_wallet_nonos/src/wallet/publish/sign.rs`) refuses with "no Bond
contract on this network yet" before it calls the keyring, and the meter's
vouchers need a channel opened on a deployed Settlement. So this boot shows
the keyring up, and the signing itself stays host-tested: wallet_proofs, 435
tests, including the verifying contract being nonos_nox_config's for every
entry and chain, NotDeployed while it is None, and a deployed address
reaching the digest. It is open until the contracts exist.

## Known, not this unit's

`ipc-late-by-server.txt`: 59 `ipc.call late` for driver.virtio_net0, 6 for
driver.virtio_gpu0, 1 for wallpaper, none for the keyring. Since B-04 every
call slower than its threshold is logged as late, and this host was running
two other builds while it booted.
