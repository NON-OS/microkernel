# nonos-stark-enroll

The host tool that builds the attestation trees and writes the v4 trailers the
gates check. A gate admits a capsule, a kernel or a bootloader only when its
measurement, with the context it runs under, is a slot of a tree whose root
the gate already holds, and the trailer proves that slot twice: a Poseidon
Merkle path, and a STARK proof of the same slot. How the gates read the
trailers is on [the STARK layer page](../docs/handbook/trust/stark.md).

## Verbs

| verb | what it does |
|---|---|
| `capsules <root.bin> <CAPS:image:trailer-out> ...` | one 256-slot tree over every capsule given, each with its capability word |
| `kernel <image> <root.bin> <trailer.bin>` | a one-slot tree over the kernel's BLAKE3 |
| `bootloader <image.efi> <root.bin> <trailer.bin>` | a one-slot tree over the loader's PE Authenticode SHA-256 |
| `verify`, `verify-kernel`, `verify-bootloader` | re-check written trailers with the gates' own check |
| `recompute <root.bin.transcript>` | rebuild a root from its transcript alone and compare |
| `refusal-variants <root.bin> <CAPS:image:trailer> <outdir>` | broken trailers for the test-only refusal profile |
| `authenticode <image.efi>` | print the digest the firmware would log for a loader |
| `selftest` | the enroll, trailer and gate loop end to end, with spliced and flipped proofs refused |

Every leaf not given a slot is a padding leaf drawn from a 32-byte seed read
from `/dev/urandom`. Each path is folded with the gate's `verify` before its
proof is made, and each finished trailer goes through the gate's full check,
path and `nox_verify`, before it is written. A trailer the boot would refuse
never leaves the tool. Beside the root it writes `<root>.transcript`: the
depth, both epochs, the pad seed, every slot and the root. Nothing in a tree
is secret.

`authenticode` and `bootloader` refuse a loader whose digest changes when
padded to 8 bytes, since signing pads the image and the firmware would then
log a value that was never enrolled.

## Proving

The slots are proven on worker threads: `NONOS_ENROLL_JOBS` when set, else a
quarter of the cores, at most 8. Each proof peaks near 3 GB. Each finished
proof prints one `progress:` line on stderr with the count, the slot, the time
spent and an estimate of the time left. Proving every capsule takes hours.

The prover `stark_proofs` (with `fri8` and `parallel`) and the verifier
`nox_verify` come from STARKs main. The commit is the flake's `starks` input,
which `tools/nonos-starks-sync` writes into `Cargo.lock`.

## Build and use

From this directory, `cargo build --release`. The flake builds it as part of
`.#host-tools`. `make` calls it for `ZK_CAPSULE_ROOT`, the kernel enrollment
stamp and the loader; the seal (`nix run .#seal`) calls `capsules`, `kernel`
and `bootloader` in that order. The roots it writes are committed under
`nonos-data/trust/policy/`, the capsule trailers under
`nonos-data/trust/capsules/`.

## What it does not do

- It has no cargo tests; `selftest` is its check.
- The epochs in every context are fixed at 1.
- It signs nothing. The release records over the roots are made by
  `tools/nonos-policy-approve` with the device policy key.
