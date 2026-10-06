# capsule_shield_vectors

Test 5 of the shield: the four pinned production wallet vectors proved on
the machine, by the same `nox_prover` that `nonos.shield` links, and every
proof held to the pinned bytes. Development images only.

```text
boot (dev image, nonos-capsule-shield-vectors on)
    |
    v
shield_vectors -- nox_prover, vectors/ (transfer and withdraw, ETH and NOX)
    |
    `-- SHIELD-VECTORS lines on the serial console, then exit
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x19
CAPSULE_OPTIONAL_CAPS = 0x100
```

- `0x001` CoreExec: run at all.
- `0x008` IPC: the prover's worker threads, through `MkThreadSpawn`.
- `0x010` Memory: the prover's heap, up to about 1.5 GB.
- `0x100` Debug, optional: the result lines on the serial console.

Its manifest names `service:4988:shield_vectors`; it serves no requests. The
kernel mirror is `src/userspace/capsule_shield_vectors`.

## Who may run it

Only a development image. `tools/nix/config.nix` refuses the feature
beside any loader but `dev-qemu`, and the kernel mirror refuses it beside
`nonos-release`. The capsule is `CAPSULE_DEV_ONLY`: no make lane signs or
enrolls it, and only the development seal enrolls it.

## What it proves

For each vector under `vectors/` (transfer-eth, transfer-nox, withdraw-eth,
withdraw-nox):

- the proof's `proof.json` is the pinned one, byte for byte;
- its format 7 form is the pinned `proof-format7.bin`, byte for byte;
- the format 7 form verifies.

The first vector proves from nothing; the rest from the periodic cache the
build ships, or from the tree the first proof built when none is shipped. A
shipped cache is also checked: it loads, and the same file with one bit of
its root changed is refused. Each proof's time, heap peak and the machine's
free memory go on the serial line.

## Keys and secrets

None. The vectors' seeds and entropy are public test inputs pinned in the
STARKs spec, not keys of any wallet.

## Verification

1. Set `features = ["nonos-capsule-shield-vectors"]` in `nonos.toml`. Do
   not commit it.
2. `make dev-image`, then `make dev-boot QEMU_ARGS="--smp 4"`.
3. The serial log ends `SHIELD-VECTORS DONE 4 of 4 vectors byte for byte`.

See `docs/TESTING-LOCALLY.md`, part 7.
