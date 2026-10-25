# The nonos-rustc fix and the keyring clean under clippy, booted on main

`head.txt`: the tree booted, main 9880aecb2 with b/rustc-check-cfg 2e9710a5e
and b/keyring-clippy 25e2af5a6 merged, `inputs.json` naming the new test
file and two module lists in rustfmt's order, and the image's sha256.
`tools/nonos-dev-image --profile qemu` sealed it with throwaway keys in its
own copy of the tree; no key under `.keys/` was read. `qemu.txt`: Intel IOMMU
lane, 4 GiB, a fresh data disk, no TPM. Setup walked by keystroke, defaults.

## The two units

- nonos-rustc (`tools/nix/nonos-rustc.py`) recomputes `-C metadata`; its
  `parse_unit` took the value of `--check-cfg`, `-l` and `--force-warn` as a
  source, so declaring a feature moved a crate's identity. It now steps over
  them, hashes `--env-set` in both spellings and reads an empty `=` value.
  Self-test OK. `elf-table.tsv`: of the 158 capsule ELFs, stripped and built
  in one directory, 140 move, all by that identity; on `hello` the
  instruction multiset is unchanged, only order and padding.
- The keyring: its 11 clippy errors gone. The seven EIP-1559 field lists go
  through one `tx_fields(&TxFields{..})`, sized once; the part tuples are
  named; `_start` has its `# Safety`. wallet_proofs 436 passed, 0 failed;
  `keyring_tx_fields_tests` holds each field to RLP written in the test, and
  swapping max_priority and max_fee fails it. `.text` +592 bytes.

## What the boot shows

- `serial.log`, 1702 lines: 68 `[SPAWN]`, 58 `[ZK-ATTEST] ok`, as main's
  qemu-dev evidence. No capsule failed, exited, faulted or was reaped in the
  six minutes the desktop was up.
- `keyring-lines.txt`: `[ZK-ATTEST] ok keyring vendor`, `[SPAWN] name=keyring
  pid=0x3 caps=0x10000038`, the run queue.
- `20-desktop.png` (3340 colours): desktop, menu bar, full dock and the live
  session dialog. `21-desktop-6min.png` (5896): six minutes on, still live.

## Checks before landing

run-static-checks PASS (333 ok); the 39 static-abi commands; `inputs.py
--check`; rustfmt with skip_children on every touched `.rs`; kernel `cargo
check`: x86_64 default 0 warnings, nonos-smp 0, aarch64 (nonos-arch-preview) 5.

## Known, not these units'

`[ATTEST-DOC] refused: tpm did not respond in time`: main's attest document
(3bff7bb85) on a run with no TPM. The NYM directory sync fails offline.
