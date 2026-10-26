# The development catalogue and the devnet route, booted on main

`head.txt`: the tree booted, main 8bf24f59b with b/dev-catalogue and
b/devnet-2 merged and the fixes below, and the image's sha256.
`tools/nonos-dev-image --profile qemu` sealed it from a fresh copy with
throwaway keys; no key under `.keys/` was read. `qemu.txt`: Intel IOMMU lane,
4 GiB, a fresh data disk, no TPM. Setup walked by keystroke with defaults.

## The two units

- dev-catalogue: a development seal in nonos-dev-image's copy signs
  `tools/devnet/market/index.json`, four listings priced on chain 31337,
  under the copy's throwaway operator key; a release seal never takes it
  (scripts/check_dev_catalogue.py, dev_catalogue_chain.py, nonos-mk tests).
- devnet-2: payment and nox-settle, in a development twin only, dial
  rpc.devnet.nonos.test at 10.0.2.2 and trust only the throwaway root in
  tools/devnet/tls, held to the exact host, serverAuth, a leaf that is no CA
  and a chain of one or two (tls_proofs 166 passed).

## Release bytes

`release-elf-table.tsv`: `.#capsules` of main 8bf24f59b against this tree,
159 ELFs, all identical stripped and unstripped. Neither unit changes a byte
a release ships.

## What the boot shows

- `22-market-list.png`: the Terminal's `market list` answers ":: 4 listings":
  devtest/free, devtest/install, devtest/metered and devtest/escrow, each
  "not ready", since no payee has attested them on a chain.
- `market-lines.txt`: keyring, payment, nox_settle and market.index each
  `[ZK-ATTEST] ok` and `[SPAWN]`.
- `serial.log`, 1763 lines: 67 `[SPAWN]`, 57 `[ZK-ATTEST] ok`. The 68 of
  main's evidence held app.install, which that walk opened from the live
  session dialog; this one closed it with Esc and opened the Terminal. No
  capsule failed, exited, faulted or was reaped in six minutes.
- `20-desktop.png` (75219 colours): the desktop, menu bar and dock.
  `23-six-min.png`: the Terminal six minutes on, still live.
- No anvil answers on 10.0.2.2 here, so no handshake is tried yet: the
  live and the refused handshake are devnet step 4.

## Fixed on the way

- tools/nonos_seal: dev-catalogue's move of the phase-1 inputs into
  market_inputs.py, with main's `--catalogue` and its four SOURCES kept.
- scripts/baselines/prebuilt.txt: tools/devnet/tls/* classed as generated.
- nonos-dev-image: a fresh copy failed at host-tools, which mounts the
  market's key file; it now holds the tracked operator public key until the
  throwaway pair replaces it.

## Known, not these units'

`[MEASURE] PCR 14 extend failed` and `[ATTEST-DOC] refused`: main's PCR 14
work (798b87e70, 3bff7bb85) on a run with no TPM.
